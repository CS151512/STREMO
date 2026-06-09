"""
Shared pytest fixtures for unit and integration tests.
Fixture scopes at a glance:
  - session  : real DB / Redis / Kafka connections (integration only)
  - function : in-memory fakes and mocked services (unit tests, default)
"""

from __future__ import annotations

import asyncio
from datetime import datetime, timedelta, timezone
from typing import AsyncIterator
from unittest.mock import AsyncMock
from uuid import uuid4

import pytest
import pytest_asyncio

from src.core.config import Settings
from src.domain.entities import RefreshToken, User, UserRole, UserStatus
from src.schemes.dto import LoginRequest, RefreshRequest, RegisterRequest


# EVENT LOOP
@pytest.fixture(scope="session")
def event_loop():
    """Single event loop for the whole test session."""
    policy = asyncio.get_event_loop_policy()
    loop = policy.new_event_loop()
    yield loop
    loop.close()


# SETTINGS OVERRIDE — ALWAYS USE TEST VALUES, NEVER PRODUCTION SECRETS
@pytest.fixture(scope="session")
def test_settings() -> Settings:
    return Settings(
        APP_ENV="development",
        DEBUG=True,
        JWT_SECRET_KEY="test-secret-key-not-for-production",
        JWT_ACCESS_TOKEN_EXPIRE_MINUTES=15,
        JWT_REFRESH_TOKEN_EXPIRE_DAYS=1,
        POSTGRES_DSN="postgresql+asyncpg://postgres:postgres@localhost:5432/auth_test",
        REDIS_DSN="redis://localhost:6379/1",
        KAFKA_BOOTSTRAP_SERVERS="localhost:9092",
        BCRYPT_ROUNDS=4,  # fast rounds for tests
    )


# DOMAIN OBJECT FACTORIES
@pytest.fixture
def make_user():
    """Factory fixture — call it to get a ready-to-use User domain object."""

    def _factory(
            email: str = "user@example.com",
            password_hash: str = "$2b$04$testhashvalue",
            role: UserRole = UserRole.USER,
            status: UserStatus = UserStatus.ACTIVE,
            first_name: str | None = "Jane",
            last_name: str | None = "Doe",
    ) -> User:
        user = User(
            id=uuid4(),
            email=email,
            hashed_password=password_hash,
            role=role,
            status=status,
            first_name=first_name,
            last_name=last_name,
        )
        return user

    return _factory


@pytest.fixture
def active_user(make_user) -> User:
    return make_user(status=UserStatus.ACTIVE)


@pytest.fixture
def inactive_user(make_user) -> User:
    return make_user(status=UserStatus.INACTIVE)


@pytest.fixture
def admin_user(make_user) -> User:
    return make_user(role=UserRole.ADMIN, status=UserStatus.ACTIVE)


@pytest.fixture
def make_refresh_token(active_user):
    """Factory for RefreshToken domain objects."""

    def _factory(
            user: User | None = None,
            token: str = "some.refresh.token",
            revoked: bool = False,
            expires_in_days: int = 7,
    ) -> RefreshToken:
        rt = RefreshToken(
            token=token,
            user_id=(user or active_user).id,
            expires_at=datetime.now(tz=timezone.utc) + timedelta(days=expires_in_days),
            revoked=revoked,
        )
        return rt

    return _factory


@pytest.fixture
def valid_refresh_token(make_refresh_token) -> RefreshToken:
    return make_refresh_token()


@pytest.fixture
def expired_refresh_token(make_refresh_token) -> RefreshToken:
    return make_refresh_token(expires_in_days=-1)


@pytest.fixture
def revoked_refresh_token(make_refresh_token) -> RefreshToken:
    return make_refresh_token(revoked=True)


# REQUEST DTO FIXTURES
@pytest.fixture
def register_request() -> RegisterRequest:
    return RegisterRequest(
        email="newuser@example.com",
        password="Str0ng!Pass",
        first_name="New",
        last_name="User",
    )


@pytest.fixture
def login_request() -> LoginRequest:
    return LoginRequest(email="user@example.com", password="Str0ng!Pass")


@pytest.fixture
def refresh_request(valid_refresh_token) -> RefreshRequest:
    return RefreshRequest(refresh_token=valid_refresh_token.token)


# INFRASTRUCTURE MOCKS (UNIT TESTS)
@pytest.fixture
def mock_user_repo():
    repo = AsyncMock()
    repo.get_by_email.return_value = None
    repo.get_by_id.return_value = None
    repo.save.return_value = None
    return repo


@pytest.fixture
def mock_token_repo():
    repo = AsyncMock()
    repo.get_by_token.return_value = None
    repo.save.return_value = None
    repo.revoke_all_for_user.return_value = None
    return repo


@pytest.fixture
def mock_redis():
    client = AsyncMock()
    client.setex.return_value = True
    client.get.return_value = None
    return client


@pytest.fixture
def mock_kafka_producer():
    producer = AsyncMock()
    producer.produce.return_value = None
    return producer


# AUTHSERVICE (UNIT TESTS — ALL DEPS ARE MOCKS)
@pytest.fixture
def auth_service(mock_user_repo, mock_token_repo, mock_redis, mock_kafka_producer):
    from src.services.auth import AuthService

    return AuthService(
        user_repo=mock_user_repo,
        token_repo=mock_token_repo,
        redis_client=mock_redis,
        kafka_producer=mock_kafka_producer,
    )


# FASTAPI TEST CLIENT (INTEGRATION TESTS)
@pytest_asyncio.fixture(scope="session")
async def test_app(test_settings):
    """
    Full FastAPI app wired to real infrastructure.
    Requires running Postgres, Redis, and Kafka (e.g. via docker-compose).
    """
    from unittest.mock import patch

    with patch("src.core.config.get_settings", return_value=test_settings):
        from src.main import create_app
        application = create_app()

    async with application.router.lifespan_context(application):
        yield application


@pytest_asyncio.fixture
async def client(test_app) -> AsyncIterator:
    from httpx import AsyncClient, ASGITransport

    async with AsyncClient(
            transport=ASGITransport(app=test_app),
            base_url="http://testserver",
    ) as ac:
        yield ac
