from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import StrEnum
from uuid import UUID, uuid4


class UserRole(StrEnum):
    ADMIN = "admin"
    USER = "user"
    SERVICE = "service"


class UserStatus(StrEnum):
    ACTIVE = "active"
    INACTIVE = "inactive"
    BANNED = "banned"
    PENDING_VERIFICATION = "pending_verification"


@dataclass
class User:
    """Core user entity — framework-agnostic"""

    email: str
    hashed_password: str

    id: UUID = field(default_factory=uuid4)
    role: UserRole = UserRole.USER
    status: UserStatus = UserStatus.PENDING_VERIFICATION
    created_at: datetime = field(default_factory=lambda: datetime.now(tz=timezone.utc))
    updated_at: datetime = field(default_factory=lambda: datetime.now(tz=timezone.utc))

    # Optional profile fields
    first_name: str | None = None
    last_name: str | None = None

    @property
    def is_active(self) -> bool:
        return self.status == UserStatus.ACTIVE

    @property
    def is_admin(self) -> bool:
        return self.role == UserRole.ADMIN

    @property
    def full_name(self) -> str | None:
        parts = filter(None, [self.first_name, self.last_name])
        return " ".join(parts) or None

    def activate(self) -> None:
        self.status = UserStatus.ACTIVE
        self._touch()

    def deactivate(self) -> None:
        self.status = UserStatus.INACTIVE
        self._touch()

    def ban(self) -> None:
        self.status = UserStatus.BANNED
        self._touch()

    def _touch(self) -> None:
        self.updated_at = datetime.now(tz=timezone.utc)


@dataclass
class RefreshToken:
    """Persisted refresh token for rotation / revocation"""

    token: str
    user_id: UUID
    expires_at: datetime

    id: UUID = field(default_factory=uuid4)
    created_at: datetime = field(default_factory=lambda: datetime.now(tz=timezone.utc))
    revoked: bool = False

    @property
    def is_expired(self) -> bool:
        return datetime.now(tz=timezone.utc) >= self.expires_at

    @property
    def is_valid(self) -> bool:
        return not self.revoked and not self.is_expired

    def revoke(self) -> None:
        self.revoked = True