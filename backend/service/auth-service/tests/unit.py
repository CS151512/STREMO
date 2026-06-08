"""
Unit tests — no real I/O, all infrastructure is mocked.
Coverage targets:
  - src/core/security.py      : JWT creation / decoding, password hashing
  - src/core/exceptions.py    : exception hierarchy and attributes
  - src/domain/entities.py    : User and RefreshToken domain logic
  - src/services/validators.py: password strength and email-domain rules
  - src/services/auth.py      : AuthService flows (register, login, refresh, logout)
"""

from __future__ import annotations

from datetime import datetime, timedelta, timezone
from unittest.mock import AsyncMock, patch
from uuid import uuid4

import pytest

from src.core.exceptions import (
    InvalidCredentialsError,
    InvalidTokenError,
    TokenExpiredError,
    UserAlreadyExistsError,
    UserInactiveError,
    WeakPasswordError,
)
from src.core.security import (
    create_access_token,
    create_refresh_token,
    decode_refresh_token,
    decode_token,
    hash_password,
    verify_password,
)
from src.domain.entities import RefreshToken, User, UserRole, UserStatus
from src.schemes.dto import LoginRequest, RefreshRequest, RegisterRequest
from src.services.validators import validate_email_domain, validate_password_strength


# core/security.py
class TestPasswordHashing:
    def test_hash_returns_string(self):
        assert isinstance(hash_password("Str0ng!Pass"), str)

    def test_hash_is_not_plaintext(self):
        assert hash_password("Str0ng!Pass") != "Str0ng!Pass"

    def test_two_hashes_differ(self):
        # bcrypt salts must differ between calls
        assert hash_password("Str0ng!Pass") != hash_password("Str0ng!Pass")

    def test_verify_correct_password(self):
        hashed = hash_password("Str0ng!Pass")
        assert verify_password("Str0ng!Pass", hashed) is True

    def test_verify_wrong_password(self):
        hashed = hash_password("Str0ng!Pass")
        assert verify_password("WrongPass1!", hashed) is False

    def test_verify_empty_string_fails(self):
        hashed = hash_password("Str0ng!Pass")
        assert verify_password("", hashed) is False


class TestJWT:
    def test_access_token_is_string(self):
        token = create_access_token(uuid4())
        assert isinstance(token, str)
        assert len(token.split(".")) == 3  # header.payload.signature

    def test_refresh_token_is_string(self):
        token = create_refresh_token(uuid4())
        assert isinstance(token, str)

    def test_decode_access_token_returns_subject(self):
        user_id = uuid4()
        token = create_access_token(user_id)
        payload = decode_token(token)
        assert payload["sub"] == str(user_id)

    def test_decode_access_token_type(self):
        token = create_access_token(uuid4())
        payload = decode_token(token)
        assert payload["type"] == "access"

    def test_decode_refresh_token_type(self):
        token = create_refresh_token(uuid4())
        payload = decode_refresh_token(token)
        assert payload["type"] == "refresh"

    def test_extra_claims_are_included(self):
        token = create_access_token(uuid4(), extra_claims={"role": "admin"})
        payload = decode_token(token)
        assert payload["role"] == "admin"

    def test_access_token_rejected_as_refresh(self):
        token = create_access_token(uuid4())
        with pytest.raises(InvalidTokenError):
            decode_refresh_token(token)

    def test_refresh_token_rejected_as_access(self):
        token = create_refresh_token(uuid4())
        with pytest.raises(InvalidTokenError):
            decode_token(token)

    def test_tampered_token_raises_invalid(self):
        token = create_access_token(uuid4())
        tampered = token[:-4] + "xxxx"
        with pytest.raises(InvalidTokenError):
            decode_token(tampered)

    def test_expired_token_raises_expired(self):
        from jose import jwt as jose_jwt
        from src.core.config import get_settings
        settings = get_settings()

        past = datetime.now(tz=timezone.utc) - timedelta(seconds=1)
        payload = {"sub": str(uuid4()), "type": "access", "exp": past}
        expired_token = jose_jwt.encode(
            payload, settings.JWT_SECRET_KEY, algorithm=settings.JWT_ALGORITHM
        )
        with pytest.raises(TokenExpiredError):
            decode_token(expired_token)

    def test_garbage_string_raises_invalid(self):
        with pytest.raises(InvalidTokenError):
            decode_token("not.a.token")


# core/exceptions.py
class TestExceptions:
    def test_default_detail_used_when_none_given(self):
        exc = InvalidCredentialsError()
        assert exc.detail == "Invalid email or password."

    def test_custom_detail_overrides_default(self):
        exc = InvalidTokenError("Custom message.")
        assert exc.detail == "Custom message."

    def test_exception_is_str_representable(self):
        exc = UserAlreadyExistsError()
        assert str(exc) == exc.detail

    def test_status_codes(self):
        from http import HTTPStatus
        assert InvalidCredentialsError.status_code == HTTPStatus.UNAUTHORIZED
        assert UserAlreadyExistsError.status_code == HTTPStatus.CONFLICT
        assert UserInactiveError.status_code == HTTPStatus.FORBIDDEN
        assert WeakPasswordError.status_code == HTTPStatus.UNPROCESSABLE_ENTITY


# domain/entities.py — User
class TestUserEntity:
    def test_default_status_is_pending(self):
        user = User(email="a@b.com", hashed_password="hash")
        assert user.status == UserStatus.PENDING_VERIFICATION

    def test_activate_sets_active(self):
        user = User(email="a@b.com", hashed_password="hash")
        user.activate()
        assert user.is_active is True
        assert user.status == UserStatus.ACTIVE

    def test_deactivate_sets_inactive(self):
        user = User(email="a@b.com", hashed_password="hash")
        user.activate()
        user.deactivate()
        assert user.is_active is False
        assert user.status == UserStatus.INACTIVE

    def test_ban_sets_banned(self):
        user = User(email="a@b.com", hashed_password="hash")
        user.ban()
        assert user.status == UserStatus.BANNED
        assert user.is_active is False

    def test_activate_updates_updated_at(self):
        user = User(email="a@b.com", hashed_password="hash")
        before = user.updated_at
        user.activate()
        assert user.updated_at >= before

    def test_is_admin_true_for_admin_role(self):
        user = User(email="a@b.com", hashed_password="hash", role=UserRole.ADMIN)
        assert user.is_admin is True

    def test_is_admin_false_for_user_role(self):
        user = User(email="a@b.com", hashed_password="hash", role=UserRole.USER)
        assert user.is_admin is False

    def test_full_name_both_parts(self):
        user = User(
            email="a@b.com", hashed_password="h",
            first_name="Jane", last_name="Doe",
        )
        assert user.full_name == "Jane Doe"

    def test_full_name_only_first(self):
        user = User(email="a@b.com", hashed_password="h", first_name="Jane")
        assert user.full_name == "Jane"

    def test_full_name_none_when_empty(self):
        user = User(email="a@b.com", hashed_password="h")
        assert user.full_name is None

    def test_unique_ids_by_default(self):
        u1 = User(email="a@b.com", hashed_password="h")
        u2 = User(email="b@b.com", hashed_password="h")
        assert u1.id != u2.id


# domain/entities.py — RefreshToken
class TestRefreshTokenEntity:
    def _make(self, *, days: int = 7, revoked: bool = False) -> RefreshToken:
        return RefreshToken(
            token="tok",
            user_id=uuid4(),
            expires_at=datetime.now(tz=timezone.utc) + timedelta(days=days),
            revoked=revoked,
        )

    def test_valid_token_is_valid(self):
        assert self._make().is_valid is True

    def test_expired_token_is_not_valid(self):
        assert self._make(days=-1).is_valid is False

    def test_revoked_token_is_not_valid(self):
        assert self._make(revoked=True).is_valid is False

    def test_revoke_sets_flag(self):
        rt = self._make()
        rt.revoke()
        assert rt.revoked is True
        assert rt.is_valid is False

    def test_is_expired_false_for_future(self):
        assert self._make(days=1).is_expired is False

    def test_is_expired_true_for_past(self):
        assert self._make(days=-1).is_expired is True


# services/validators.py
class TestPasswordValidator:
    @pytest.mark.parametrize("pw", [
        "Str0ng!Pass",
        "C0mpl3x@Word",
        "V3ryS3cur3#",
        "A1b2C3d4!",
    ])
    def test_valid_passwords_pass(self, pw: str):
        validate_password_strength(pw)  # should not raise

    @pytest.mark.parametrize("pw,reason", [
        ("short1A!", "too short"),
        ("alllowercase1!", "no uppercase"),
        ("ALLUPPERCASE1!", "no lowercase"),
        ("NoDigitsHere!", "no digit"),
        ("NoSpecial1Char", "no special char"),
        ("Password1!", "common password"),
    ])
    def test_invalid_passwords_raise(self, pw: str, reason: str):
        with pytest.raises(WeakPasswordError, match=""), \
             pytest.raises(WeakPasswordError):
            validate_password_strength(pw)

    def test_common_password_specific_message(self):
        with pytest.raises(WeakPasswordError, match="too common"):
            validate_password_strength("Password1!")

    def test_password_at_max_length_passes(self):
        # 128 chars, meets all rules
        pw = "Aa1!" + "x" * 124
        validate_password_strength(pw)

    def test_password_over_max_length_fails(self):
        pw = "Aa1!" + "x" * 125  # 129 chars
        with pytest.raises(WeakPasswordError):
            validate_password_strength(pw)


class TestEmailDomainValidator:
    def test_no_blocked_domains_passes(self):
        validate_email_domain("user@any.com", blocked_domains=None)

    def test_allowed_domain_passes(self):
        validate_email_domain("user@gmail.com", blocked_domains={"tempmail.com"})

    def test_blocked_domain_raises(self):
        from src.core.exceptions import ValidationException
        with pytest.raises(ValidationException):
            validate_email_domain("user@tempmail.com", blocked_domains={"tempmail.com"})

    def test_domain_check_is_case_insensitive(self):
        from src.core.exceptions import ValidationException
        with pytest.raises(ValidationException):
            validate_email_domain("user@TempMail.COM", blocked_domains={"tempmail.com"})


# services/auth.py — AuthService
@pytest.mark.asyncio
class TestAuthServiceRegister:
    async def test_register_returns_user_and_tokens(
        self, auth_service, register_request, mock_user_repo
    ):
        mock_user_repo.get_by_email.return_value = None
        result = await auth_service.register(register_request)

        assert result.user.email == register_request.email
        assert result.tokens.access_token
        assert result.tokens.refresh_token

    async def test_register_saves_user(
        self, auth_service, register_request, mock_user_repo
    ):
        mock_user_repo.get_by_email.return_value = None
        await auth_service.register(register_request)
        mock_user_repo.save.assert_awaited_once()

    async def test_register_persists_refresh_token(
        self, auth_service, register_request, mock_token_repo
    ):
        await auth_service.register(register_request)
        mock_token_repo.save.assert_awaited()

    async def test_register_publishes_kafka_event(
        self, auth_service, register_request, mock_kafka_producer
    ):
        await auth_service.register(register_request)
        mock_kafka_producer.produce.assert_awaited_once()

    async def test_register_raises_if_email_taken(
        self, auth_service, register_request, mock_user_repo, active_user
    ):
        mock_user_repo.get_by_email.return_value = active_user
        with pytest.raises(UserAlreadyExistsError):
            await auth_service.register(register_request)

    async def test_register_raises_on_weak_password(
        self, auth_service, mock_user_repo
    ):
        mock_user_repo.get_by_email.return_value = None
        weak_req = RegisterRequest(email="x@example.com", password="weakpass")
        with pytest.raises(WeakPasswordError):
            await auth_service.register(weak_req)

    async def test_register_stores_hashed_password(
        self, auth_service, register_request, mock_user_repo
    ):
        mock_user_repo.get_by_email.return_value = None
        await auth_service.register(register_request)

        saved_user: User = mock_user_repo.save.call_args[0][0]
        assert saved_user.hashed_password != register_request.password

    async def test_register_activates_user(
        self, auth_service, register_request, mock_user_repo
    ):
        mock_user_repo.get_by_email.return_value = None
        result = await auth_service.register(register_request)
        assert result.user.status == UserStatus.ACTIVE


@pytest.mark.asyncio
class TestAuthServiceLogin:
    async def test_login_returns_token_pair(
        self, auth_service, login_request, active_user, mock_user_repo
    ):
        active_user.hashed_password = hash_password(login_request.password)
        mock_user_repo.get_by_email.return_value = active_user

        result = await auth_service.login(login_request)
        assert result.access_token
        assert result.refresh_token

    async def test_login_wrong_password_raises(
        self, auth_service, login_request, active_user, mock_user_repo
    ):
        active_user.hashed_password = hash_password("DifferentP@ss1")
        mock_user_repo.get_by_email.return_value = active_user

        with pytest.raises(InvalidCredentialsError):
            await auth_service.login(login_request)

    async def test_login_unknown_email_raises(
        self, auth_service, login_request, mock_user_repo
    ):
        mock_user_repo.get_by_email.return_value = None
        with pytest.raises(InvalidCredentialsError):
            await auth_service.login(login_request)

    async def test_login_inactive_user_raises(
        self, auth_service, login_request, inactive_user, mock_user_repo
    ):
        inactive_user.hashed_password = hash_password(login_request.password)
        mock_user_repo.get_by_email.return_value = inactive_user

        with pytest.raises(UserInactiveError):
            await auth_service.login(login_request)

    async def test_login_persists_refresh_token(
        self, auth_service, login_request, active_user, mock_user_repo, mock_token_repo
    ):
        active_user.hashed_password = hash_password(login_request.password)
        mock_user_repo.get_by_email.return_value = active_user

        await auth_service.login(login_request)
        mock_token_repo.save.assert_awaited()


@pytest.mark.asyncio
class TestAuthServiceRefresh:
    async def test_refresh_returns_new_access_token(
        self,
        auth_service,
        active_user,
        valid_refresh_token,
        mock_user_repo,
        mock_token_repo,
    ):
        real_refresh = create_refresh_token(active_user.id)
        valid_refresh_token.token = real_refresh

        mock_token_repo.get_by_token.return_value = valid_refresh_token
        mock_user_repo.get_by_id.return_value = active_user

        result = await auth_service.refresh(RefreshRequest(refresh_token=real_refresh))
        assert result.access_token

    async def test_refresh_revokes_old_token(
        self,
        auth_service,
        active_user,
        valid_refresh_token,
        mock_user_repo,
        mock_token_repo,
    ):
        real_refresh = create_refresh_token(active_user.id)
        valid_refresh_token.token = real_refresh

        mock_token_repo.get_by_token.return_value = valid_refresh_token
        mock_user_repo.get_by_id.return_value = active_user

        await auth_service.refresh(RefreshRequest(refresh_token=real_refresh))
        assert valid_refresh_token.revoked is True

    async def test_refresh_blacklists_old_token_in_redis(
        self,
        auth_service,
        active_user,
        valid_refresh_token,
        mock_user_repo,
        mock_token_repo,
        mock_redis,
    ):
        real_refresh = create_refresh_token(active_user.id)
        valid_refresh_token.token = real_refresh

        mock_token_repo.get_by_token.return_value = valid_refresh_token
        mock_user_repo.get_by_id.return_value = active_user

        await auth_service.refresh(RefreshRequest(refresh_token=real_refresh))
        mock_redis.setex.assert_awaited_once()
        key = mock_redis.setex.call_args[0][0]
        assert key.startswith("blacklist:")

    async def test_refresh_unknown_token_raises(
        self, auth_service, active_user, mock_token_repo
    ):
        real_refresh = create_refresh_token(active_user.id)
        mock_token_repo.get_by_token.return_value = None

        with pytest.raises(InvalidTokenError):
            await auth_service.refresh(RefreshRequest(refresh_token=real_refresh))

    async def test_refresh_revoked_token_raises_and_revokes_all(
        self,
        auth_service,
        active_user,
        revoked_refresh_token,
        mock_user_repo,
        mock_token_repo,
    ):
        real_refresh = create_refresh_token(active_user.id)
        revoked_refresh_token.token = real_refresh

        mock_token_repo.get_by_token.return_value = revoked_refresh_token

        with pytest.raises(InvalidTokenError):
            await auth_service.refresh(RefreshRequest(refresh_token=real_refresh))

        mock_token_repo.revoke_all_for_user.assert_awaited_once_with(active_user.id)

    async def test_refresh_inactive_user_raises(
        self,
        auth_service,
        inactive_user,
        valid_refresh_token,
        mock_user_repo,
        mock_token_repo,
    ):
        real_refresh = create_refresh_token(inactive_user.id)
        valid_refresh_token.token = real_refresh

        mock_token_repo.get_by_token.return_value = valid_refresh_token
        mock_user_repo.get_by_id.return_value = inactive_user

        with pytest.raises(UserInactiveError):
            await auth_service.refresh(RefreshRequest(refresh_token=real_refresh))

    async def test_refresh_with_garbage_jwt_raises(self, auth_service):
        with pytest.raises(InvalidTokenError):
            await auth_service.refresh(RefreshRequest(refresh_token="garbage"))


@pytest.mark.asyncio
class TestAuthServiceLogout:
    async def test_logout_revokes_stored_token(
        self, auth_service, valid_refresh_token, mock_token_repo
    ):
        mock_token_repo.get_by_token.return_value = valid_refresh_token
        await auth_service.logout(valid_refresh_token.token)
        assert valid_refresh_token.revoked is True

    async def test_logout_blacklists_token(
        self, auth_service, valid_refresh_token, mock_token_repo, mock_redis
    ):
        mock_token_repo.get_by_token.return_value = valid_refresh_token
        await auth_service.logout(valid_refresh_token.token)
        mock_redis.setex.assert_awaited_once()

    async def test_logout_unknown_token_does_not_raise(
        self, auth_service, mock_token_repo, mock_redis
    ):
        mock_token_repo.get_by_token.return_value = None
        await auth_service.logout("unknown-token")  # should complete silently
        mock_redis.setex.assert_awaited_once()

    async def test_logout_all_delegates_to_repo(
        self, auth_service, active_user, mock_token_repo
    ):
        await auth_service.logout_all(active_user.id)
        mock_token_repo.revoke_all_for_user.assert_awaited_once_with(active_user.id)