from datetime import datetime, timedelta, timezone
from typing import Optional, Tuple

import bcrypt
import jwt
from jwt.exceptions import ExpiredSignatureError, InvalidTokenError as JWTInvalidError

from core.config import get_settings
from core.exceptions import TokenExpiredError, InvalidTokenError

settings = get_settings()


class PasswordHasher:
    """Handle password hashing and verification"""

    @staticmethod
    def hash_password(password: str) -> str:
        """Hash a password using bcrypt"""
        salt = bcrypt.gensalt(rounds=settings.BCRYPT_ROUNDS)
        hashed = bcrypt.hashpw(password.encode("utf-8"), salt)
        return hashed.decode("utf-8")

    @staticmethod
    def verify_password(plain_password: str, hashed_password: str) -> bool:
        """Verify a password against its hash"""
        return bcrypt.checkpw(
            plain_password.encode("utf-8"),
            hashed_password.encode("utf-8"),
        )


class JWTService:
    """Handle JWT token generation and verification"""

    @staticmethod
    def create_access_token(user_id: str, email: str) -> str:
        """Create a new access token"""
        expire = datetime.now(timezone.utc) + timedelta(minutes=settings.ACCESS_TOKEN_EXPIRE_MINUTES)
        payload = {
            "sub": user_id,
            "email": email,
            "type": "access",
            "exp": expire,
            "iat": datetime.now(timezone.utc),
        }
        return jwt.encode(payload, settings.JWT_SECRET_KEY, algorithm=settings.JWT_ALGORITHM)

    @staticmethod
    def create_refresh_token(user_id: str, email: str) -> str:
        """Create a new refresh token"""
        expire = datetime.now(timezone.utc) + timedelta(days=settings.REFRESH_TOKEN_EXPIRE_DAYS)
        payload = {
            "sub": user_id,
            "email": email,
            "type": "refresh",
            "exp": expire,
            "iat": datetime.now(timezone.utc),
        }
        return jwt.encode(payload, settings.JWT_SECRET_KEY, algorithm=settings.JWT_ALGORITHM)

    @staticmethod
    def create_tokens(user_id: str, email: str) -> Tuple[str, str]:
        """Create both access and refresh tokens"""
        access_token = JWTService.create_access_token(user_id, email)
        refresh_token = JWTService.create_refresh_token(user_id, email)
        return access_token, refresh_token

    @staticmethod
    def verify_token(token: str, token_type: str = "access") -> dict:
        """Verify and decode a JWT token"""
        try:
            payload = jwt.decode(
                token,
                settings.JWT_SECRET_KEY,
                algorithms=[settings.JWT_ALGORITHM],
            )

            if payload.get("type") != token_type:
                raise InvalidTokenError(f"Invalid token type: expected {token_type}")

            return payload
        except ExpiredSignatureError:
            raise TokenExpiredError()
        except JWTInvalidError:
            raise InvalidTokenError()

    @staticmethod
    def refresh_access_token(refresh_token: str) -> Tuple[str, str]:
        """Refresh access token using a valid refresh token"""
        payload = JWTService.verify_token(refresh_token, token_type="refresh")
        user_id = payload.get("sub")
        email = payload.get("email")

        if not user_id or not email:
            raise InvalidTokenError("Invalid token payload")

        return JWTService.create_tokens(user_id, email)

    @staticmethod
    def extract_user_id_from_token(token: str) -> str:
        """Extract user ID from access token"""
        payload = JWTService.verify_token(token, token_type="access")
        user_id = payload.get("sub")

        if not user_id:
            raise InvalidTokenError("Missing user ID in token")

        return user_id