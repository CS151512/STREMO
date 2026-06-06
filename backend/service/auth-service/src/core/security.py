"""Security utilities: password hashing, JWT tokens."""

from datetime import datetime, timedelta, timezone
from typing import Optional, Dict, Any

import bcrypt
import jwt
from jwt.exceptions import InvalidTokenError as JWTInvalidTokenError

from .config import settings
from .exceptions import InvalidCredentialsError, InvalidTokenError, TokenExpiredError


def hash_password(password: str) -> str:
    """
    Hash password using bcrypt.
    Args:
        password: Plain text password.
    Returns:
        Hashed password as string.
    """
    salt = bcrypt.gensalt(rounds=settings.bcrypt_rounds)
    return bcrypt.hashpw(password.encode("utf-8"), salt).decode("utf-8")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    """
    Verify password against hash.
    Args:
        plain_password: Plain text password.
        hashed_password: Stored hash.
    Returns:
        True if password matches, False otherwise.
    """
    return bcrypt.checkpw(
        plain_password.encode("utf-8"),
        hashed_password.encode("utf-8")
    )


def create_access_token(user_id: str, extra_data: Optional[Dict[str, Any]] = None) -> str:
    """
    Create JWT access token.
    Args:
        user_id: User identifier.
        extra_data: Additional claims to include.
    Returns:
        Encoded JWT token.
    """
    payload = {
        "sub": user_id,
        "type": "access",
        "iat": datetime.now(timezone.utc),
        "exp": datetime.now(timezone.utc) + timedelta(minutes=settings.jwt_access_token_expire_minutes),
    }

    if extra_data:
        payload.update(extra_data)

    return jwt.encode(payload, settings.jwt_secret_key, algorithm=settings.jwt_algorithm)


def create_refresh_token(user_id: str) -> str:
    """
    Create JWT refresh token.
    Args:
        user_id: User identifier.
    Returns:
        Encoded JWT refresh token.
    """
    payload = {
        "sub": user_id,
        "type": "refresh",
        "iat": datetime.now(timezone.utc),
        "exp": datetime.now(timezone.utc) + timedelta(days=settings.jwt_refresh_token_expire_days),
    }

    return jwt.encode(payload, settings.jwt_secret_key, algorithm=settings.jwt_algorithm)


def verify_token(token: str, token_type: str = "access") -> Dict[str, Any]:
    """
    Verify and decode JWT token.
    Args:
        token: JWT token string.
        token_type: Expected token type ("access" or "refresh").
    Returns:
        Decoded token payload.
    Raises:
        TokenExpiredError: If token has expired.
        InvalidTokenError: If token is invalid or wrong type.
    """
    try:
        payload = jwt.decode(
            token,
            settings.jwt_secret_key,
            algorithms=[settings.jwt_algorithm],
        )
    except jwt.ExpiredSignatureError:
        raise TokenExpiredError("Token has expired")
    except JWTInvalidTokenError as e:
        raise InvalidTokenError(f"Invalid token: {str(e)}")

    # Check token type
    if payload.get("type") != token_type:
        raise InvalidTokenError(f"Invalid token type, expected {token_type}")

    return payload


def get_user_id_from_token(token: str, token_type: str = "access") -> str:
    """
    Extract user ID from token.
    Args:
        token: JWT token string.
        token_type: Expected token type.
    Returns:
        User ID from token subject.
    """
    payload = verify_token(token, token_type)
    user_id = payload.get("sub")

    if not user_id:
        raise InvalidTokenError("Token missing subject claim")

    return user_id