from datetime import datetime
from uuid import UUID

from pydantic import BaseModel, EmailStr, Field, field_validator

from src.domain.entities import UserRole, UserStatus


# SHARED BASE
class CamelModel(BaseModel):
    """Base model that serialises to camelCase for HTTP responses."""

    model_config = {"populate_by_name": True, "use_enum_values": True}


# AUTH REQUESTS
class RegisterRequest(CamelModel):
    email: EmailStr
    password: str = Field(min_length=8, max_length=128)
    first_name: str | None = Field(default=None, max_length=64)
    last_name: str | None = Field(default=None, max_length=64)

    @field_validator("password")
    @classmethod
    def password_not_blank(cls, v: str) -> str:
        if v.strip() == "":
            raise ValueError("Password must not be blank.")
        return v


class LoginRequest(CamelModel):
    email: EmailStr
    password: str


class RefreshRequest(CamelModel):
    refresh_token: str


class LogoutRequest(CamelModel):
    refresh_token: str


# AUTH RESPONSES
class TokenPair(CamelModel):
    access_token: str
    refresh_token: str
    token_type: str = "bearer"
    expires_in: int  # seconds until access token expires


class AccessTokenResponse(CamelModel):
    access_token: str
    token_type: str = "bearer"
    expires_in: int


# USER RESPONSES
class UserResponse(CamelModel):
    id: UUID
    email: EmailStr
    role: UserRole
    status: UserStatus
    first_name: str | None
    last_name: str | None
    created_at: datetime
    updated_at: datetime

    @property
    def full_name(self) -> str | None:
        parts = filter(None, [self.first_name, self.last_name])
        return " ".join(parts) or None


class RegisterResponse(CamelModel):
    user: UserResponse
    tokens: TokenPair


# SERVICE-TO-SERVICE DTOS

class TokenPayload(CamelModel):
    """Decoded JWT payload passed between internal layers."""

    sub: UUID
    type: str
    iat: datetime
    exp: datetime
    role: UserRole = UserRole.USER


class UserCreatedEvent(CamelModel):
    """Kafka event emitted after successful registration."""

    event: str = "user.created"
    user_id: UUID
    email: EmailStr
    occurred_at: datetime