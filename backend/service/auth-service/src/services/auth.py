from datetime import datetime, timedelta, timezone
from uuid import UUID

from src.core.config import get_settings
from src.core.exceptions import (
    InvalidCredentialsError,
    InvalidTokenError,
    UserAlreadyExistsError,
    UserInactiveError,
    UserNotFoundError,
)
from src.core.security import (
    create_access_token,
    create_refresh_token,
    decode_refresh_token,
    hash_password,
    verify_password,
)
from src.domain.entities import RefreshToken, User
from src.schemes.dto import (
    AccessTokenResponse,
    LoginRequest,
    RefreshRequest,
    RegisterRequest,
    RegisterResponse,
    TokenPair,
    UserCreatedEvent,
    UserResponse,
)
from src.services.validators import validate_password_strength

settings = get_settings()


class AuthService:
    """
    Orchestrates authentication flows:
      - register, login, token refresh, logout
    Depends on abstract repository and infrastructure interfaces so it
    remains fully unit-testable without real I/O.
    """

    def __init__(
            self,
            user_repo,  # repositories.user.UserRepository
            token_repo,  # repositories.base.BaseRepository[RefreshToken]
            redis_client,  # infrastructure.redis.client.RedisClient
            kafka_producer,  # infrastructure.kafka.producer.KafkaProducer
    ) -> None:
        self._users = user_repo
        self._tokens = token_repo
        self._redis = redis_client
        self._kafka = kafka_producer

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    async def register(self, data: RegisterRequest) -> RegisterResponse:
        """Create a new user account and return a token pair."""
        await self._assert_email_available(data.email)
        validate_password_strength(data.password)

        user = User(
            email=data.email,
            hashed_password=hash_password(data.password),
            first_name=data.first_name,
            last_name=data.last_name,
        )
        user.activate()
        await self._users.save(user)

        tokens = await self._issue_token_pair(user)
        await self._publish_user_created(user)

        return RegisterResponse(
            user=self._to_user_response(user),
            tokens=tokens,
        )

    async def login(self, data: LoginRequest) -> TokenPair:
        """Authenticate with email + password and return a token pair."""
        user = await self._users.get_by_email(data.email)
        if user is None or not verify_password(data.password, user.hashed_password):
            raise InvalidCredentialsError()

        if not user.is_active:
            raise UserInactiveError()

        return await self._issue_token_pair(user)

    async def refresh(self, data: RefreshRequest) -> AccessTokenResponse:
        """
        Rotate refresh token and return a new access token.

        Uses a one-time-use pattern: the old refresh token is revoked and a
        new one is persisted in its place (refresh token rotation).
        """
        payload = decode_refresh_token(data.refresh_token)
        user_id: UUID = UUID(payload["sub"])

        stored: RefreshToken | None = await self._tokens.get_by_token(data.refresh_token)
        if stored is None or not stored.is_valid:
            # Possible token reuse attack — revoke all tokens for this user.
            await self._tokens.revoke_all_for_user(user_id)
            raise InvalidTokenError("Refresh token is invalid or has been revoked.")

        user = await self._users.get_by_id(user_id)
        if user is None:
            raise UserNotFoundError()
        if not user.is_active:
            raise UserInactiveError()

        stored.revoke()
        await self._tokens.save(stored)

        new_access = create_access_token(user.id, extra_claims={"role": user.role})
        new_refresh = create_refresh_token(user.id)
        await self._persist_refresh_token(user.id, new_refresh)

        # Blacklist the old token in Redis (belt-and-suspenders)
        await self._blacklist_token(data.refresh_token)

        return AccessTokenResponse(
            access_token=new_access,
            expires_in=settings.JWT_ACCESS_TOKEN_EXPIRE_MINUTES * 60,
        )

    async def logout(self, refresh_token: str) -> None:
        """Revoke the supplied refresh token."""
        stored: RefreshToken | None = await self._tokens.get_by_token(refresh_token)
        if stored is not None:
            stored.revoke()
            await self._tokens.save(stored)
        await self._blacklist_token(refresh_token)

    async def logout_all(self, user_id: UUID) -> None:
        """Revoke all active sessions for *user_id*."""
        await self._tokens.revoke_all_for_user(user_id)

    # ------------------------------------------------------------------
    # Private helpers
    # ------------------------------------------------------------------

    async def _assert_email_available(self, email: str) -> None:
        existing = await self._users.get_by_email(email)
        if existing is not None:
            raise UserAlreadyExistsError()

    async def _issue_token_pair(self, user: User) -> TokenPair:
        access = create_access_token(user.id, extra_claims={"role": user.role})
        refresh = create_refresh_token(user.id)
        await self._persist_refresh_token(user.id, refresh)
        return TokenPair(
            access_token=access,
            refresh_token=refresh,
            expires_in=settings.JWT_ACCESS_TOKEN_EXPIRE_MINUTES * 60,
        )

    async def _persist_refresh_token(self, user_id: UUID, token: str) -> None:
        expires_at = datetime.now(tz=timezone.utc) + timedelta(
            days=settings.JWT_REFRESH_TOKEN_EXPIRE_DAYS
        )
        record = RefreshToken(token=token, user_id=user_id, expires_at=expires_at)
        await self._tokens.save(record)

    async def _blacklist_token(self, token: str) -> None:
        ttl = settings.JWT_REFRESH_TOKEN_EXPIRE_DAYS * 86_400
        await self._redis.setex(f"blacklist:{token}", ttl, "1")

    async def _publish_user_created(self, user: User) -> None:
        event = UserCreatedEvent(
            user_id=user.id,
            email=user.email,
            occurred_at=user.created_at,
        )
        await self._kafka.produce(
            topic=settings.KAFKA_TOPIC_USER_EVENTS,
            key=str(user.id),
            value=event.model_dump_json(),
        )

    @staticmethod
    def _to_user_response(user: User) -> UserResponse:
        return UserResponse(
            id=user.id,
            email=user.email,
            role=user.role,
            status=user.status,
            first_name=user.first_name,
            last_name=user.last_name,
            created_at=user.created_at,
            updated_at=user.updated_at,
        )
