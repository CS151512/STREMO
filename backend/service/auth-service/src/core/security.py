from datetime import datetime, timedelta, timezone
from typing import Any
from uuid import UUID

import bcrypt
from jose import JWTError, jwt

from src.core.config import get_settings
from src.core.exceptions import InvalidTokenError, TokenExpiredError

settings = get_settings()


# PASSWORD HASHING
def hash_password(plain: str) -> str:
    """Return a bcrypt hash of *plain*."""
    salt = bcrypt.gensalt(rounds=settings.BCRYPT_ROUNDS)
    return bcrypt.hashpw(plain.encode(), salt).decode()


def verify_password(plain: str, hashed: str) -> bool:
    """Return True if *plain* matches *hashed*."""
    return bcrypt.checkpw(plain.encode(), hashed.encode())


# JWT
_ACCESS = "access"
_REFRESH = "refresh"


def _create_token(
        subject: str | UUID,
        token_type: str,
        expires_delta: timedelta,
        extra_claims: dict[str, Any] | None = None,
) -> str:
    now = datetime.now(tz=timezone.utc)
    payload: dict[str, Any] = {
        "sub": str(subject),
        "type": token_type,
        "iat": now,
        "exp": now + expires_delta,
    }
    if extra_claims:
        payload.update(extra_claims)
    return jwt.encode(payload, settings.JWT_SECRET_KEY, algorithm=settings.JWT_ALGORITHM)


def create_access_token(user_id: str | UUID, extra_claims: dict[str, Any] | None = None) -> str:
    return _create_token(
        subject=user_id,
        token_type=_ACCESS,
        expires_delta=timedelta(minutes=settings.JWT_ACCESS_TOKEN_EXPIRE_MINUTES),
        extra_claims=extra_claims,
    )


def create_refresh_token(user_id: str | UUID) -> str:
    return _create_token(
        subject=user_id,
        token_type=_REFRESH,
        expires_delta=timedelta(days=settings.JWT_REFRESH_TOKEN_EXPIRE_DAYS),
    )


def decode_token(token: str, expected_type: str = _ACCESS) -> dict[str, Any]:
    """
    Decode and validate *token*.
    Raises:
        TokenExpiredError: if the token is past its expiry.
        InvalidTokenError: if the token is malformed or has an unexpected type.
    """
    try:
        payload = jwt.decode(
            token,
            settings.JWT_SECRET_KEY,
            algorithms=[settings.JWT_ALGORITHM],
        )
    except jwt.ExpiredSignatureError:
        raise TokenExpiredError()
    except JWTError:
        raise InvalidTokenError()

    if payload.get("type") != expected_type:
        raise InvalidTokenError("Unexpected token type.")

    return payload


def decode_refresh_token(token: str) -> dict[str, Any]:
    return decode_token(token, expected_type=_REFRESH)
