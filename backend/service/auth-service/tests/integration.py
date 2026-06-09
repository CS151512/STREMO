"""
Integration tests — exercise the full HTTP stack against real infrastructure.
Prerequisites (e.g. via docker-compose):
  - PostgreSQL on localhost:5432  (db: auth_test)   ЭТИ ХОСТЫ НЕ Я ПРИДУМЫВАЛ
  - Redis     on localhost:6379   (db: 1)           ЭТИ ХОСТЫ НЕ Я ПРИДУМЫВАЛ
  - Kafka     on localhost:9092                     ЭТИ ХОСТЫ НЕ Я ПРИДУМЫВАЛ
Run only integration tests:
    pytest tests/integration.py -m integration
All tests are marked with @pytest.mark.integration so they can be excluded
from fast CI runs with:  pytest -m "not integration"
"""

from __future__ import annotations

import pytest
from httpx import AsyncClient

pytestmark = pytest.mark.integration


# HELPERS
_BASE = "/api/v1"

_VALID_USER = {
    "email": "integration@example.com",
    "password": "Integr@tion1",
    "firstName": "Integration",
    "lastName": "Test",
}

_OTHER_USER = {
    "email": "other@example.com",
    "password": "0th3rUs3r!",
}


async def _register(client: AsyncClient, payload: dict | None = None) -> dict:
    resp = await client.post(f"{_BASE}/auth/register", json=payload or _VALID_USER)
    assert resp.status_code == 201, resp.text
    return resp.json()


async def _login(client: AsyncClient, payload: dict | None = None) -> dict:
    body = payload or {"email": _VALID_USER["email"], "password": _VALID_USER["password"]}
    resp = await client.post(f"{_BASE}/auth/login", json=body)
    assert resp.status_code == 200, resp.text
    return resp.json()


# HEALTH
@pytest.mark.asyncio
async def test_health_endpoint(client: AsyncClient):
    resp = await client.get("/health")
    assert resp.status_code == 200
    body = resp.json()
    assert body["status"] == "ok"
    assert "version" in body


# POST /auth/register
@pytest.mark.asyncio
class TestRegister:
    async def test_register_success(self, client: AsyncClient):
        resp = await client.post(f"{_BASE}/auth/register", json=_VALID_USER)
        assert resp.status_code == 201
        body = resp.json()
        assert body["user"]["email"] == _VALID_USER["email"]
        assert "accessToken" in body["tokens"]
        assert "refreshToken" in body["tokens"]

    async def test_register_duplicate_email_returns_409(self, client: AsyncClient):
        await _register(client)
        resp = await client.post(f"{_BASE}/auth/register", json=_VALID_USER)
        assert resp.status_code == 409

    async def test_register_invalid_email_returns_422(self, client: AsyncClient):
        payload = {**_VALID_USER, "email": "not-an-email"}
        resp = await client.post(f"{_BASE}/auth/register", json=payload)
        assert resp.status_code == 422

    async def test_register_weak_password_returns_422(self, client: AsyncClient):
        payload = {**_VALID_USER, "email": "weak@example.com", "password": "weak"}
        resp = await client.post(f"{_BASE}/auth/register", json=payload)
        assert resp.status_code == 422

    async def test_register_missing_email_returns_422(self, client: AsyncClient):
        payload = {"password": "Str0ng!Pass"}
        resp = await client.post(f"{_BASE}/auth/register", json=payload)
        assert resp.status_code == 422

    async def test_register_missing_password_returns_422(self, client: AsyncClient):
        payload = {"email": "nopass@example.com"}
        resp = await client.post(f"{_BASE}/auth/register", json=payload)
        assert resp.status_code == 422

    async def test_register_response_has_no_password_field(self, client: AsyncClient):
        body = await _register(client, {**_VALID_USER, "email": "safe@example.com"})
        user_keys = body["user"].keys()
        assert "password" not in user_keys
        assert "hashedPassword" not in user_keys

    async def test_register_token_type_is_bearer(self, client: AsyncClient):
        body = await _register(client, {**_VALID_USER, "email": "bearer@example.com"})
        assert body["tokens"]["tokenType"] == "bearer"


# POST /auth/login
@pytest.mark.asyncio
class TestLogin:
    async def test_login_success(self, client: AsyncClient):
        await _register(client)
        resp = await client.post(
            f"{_BASE}/auth/login",
            json={"email": _VALID_USER["email"], "password": _VALID_USER["password"]},
        )
        assert resp.status_code == 200
        body = resp.json()
        assert "accessToken" in body
        assert "refreshToken" in body

    async def test_login_wrong_password_returns_401(self, client: AsyncClient):
        await _register(client)
        resp = await client.post(
            f"{_BASE}/auth/login",
            json={"email": _VALID_USER["email"], "password": "Wr0ng!Pass"},
        )
        assert resp.status_code == 401

    async def test_login_unknown_email_returns_401(self, client: AsyncClient):
        resp = await client.post(
            f"{_BASE}/auth/login",
            json={"email": "ghost@example.com", "password": "Str0ng!Pass"},
        )
        assert resp.status_code == 401

    async def test_login_returns_expires_in(self, client: AsyncClient):
        await _register(client)
        body = await _login(client)
        assert isinstance(body["expiresIn"], int)
        assert body["expiresIn"] > 0

    async def test_login_missing_fields_returns_422(self, client: AsyncClient):
        resp = await client.post(f"{_BASE}/auth/login", json={"email": "x@x.com"})
        assert resp.status_code == 422


# POST /auth/refresh
@pytest.mark.asyncio
class TestRefresh:
    async def test_refresh_returns_new_access_token(self, client: AsyncClient):
        await _register(client)
        tokens = await _login(client)

        resp = await client.post(
            f"{_BASE}/auth/refresh",
            json={"refreshToken": tokens["refreshToken"]},
        )
        assert resp.status_code == 200
        body = resp.json()
        assert "accessToken" in body
        # New access token must differ from the original
        assert body["accessToken"] != tokens["accessToken"]

    async def test_refresh_token_rotation(self, client: AsyncClient):
        """Using the same refresh token twice must fail on the second attempt."""
        await _register(client, {**_VALID_USER, "email": "rotation@example.com"})
        tokens = await _login(client, {"email": "rotation@example.com", "password": _VALID_USER["password"]})
        refresh_token = tokens["refreshToken"]

        # First use — valid
        resp1 = await client.post(
            f"{_BASE}/auth/refresh", json={"refreshToken": refresh_token}
        )
        assert resp1.status_code == 200

        # Second use — must be rejected (token was rotated)
        resp2 = await client.post(
            f"{_BASE}/auth/refresh", json={"refreshToken": refresh_token}
        )
        assert resp2.status_code == 401

    async def test_refresh_with_invalid_token_returns_401(self, client: AsyncClient):
        resp = await client.post(
            f"{_BASE}/auth/refresh", json={"refreshToken": "not.a.real.token"}
        )
        assert resp.status_code == 401

    async def test_refresh_with_access_token_returns_401(self, client: AsyncClient):
        await _register(client)
        tokens = await _login(client)
        # Pass the access token where a refresh token is expected
        resp = await client.post(
            f"{_BASE}/auth/refresh", json={"refreshToken": tokens["accessToken"]}
        )
        assert resp.status_code == 401

    async def test_refresh_missing_token_returns_422(self, client: AsyncClient):
        resp = await client.post(f"{_BASE}/auth/refresh", json={})
        assert resp.status_code == 422


# POST /auth/logout
@pytest.mark.asyncio
class TestLogout:
    async def test_logout_success(self, client: AsyncClient):
        await _register(client)
        tokens = await _login(client)

        resp = await client.post(
            f"{_BASE}/auth/logout",
            json={"refreshToken": tokens["refreshToken"]},
        )
        assert resp.status_code == 204

    async def test_logout_invalidates_refresh_token(self, client: AsyncClient):
        await _register(client, {**_VALID_USER, "email": "logout@example.com"})
        tokens = await _login(client, {"email": "logout@example.com", "password": _VALID_USER["password"]})
        refresh_token = tokens["refreshToken"]

        await client.post(f"{_BASE}/auth/logout", json={"refreshToken": refresh_token})

        # Subsequent refresh with the now-revoked token must fail
        resp = await client.post(
            f"{_BASE}/auth/refresh", json={"refreshToken": refresh_token}
        )
        assert resp.status_code == 401

    async def test_logout_unknown_token_still_returns_204(self, client: AsyncClient):
        """Logout is idempotent — unknown tokens are silently accepted."""
        resp = await client.post(
            f"{_BASE}/auth/logout", json={"refreshToken": "unknown.token.value"}
        )
        assert resp.status_code == 204


# GET /auth/me
@pytest.mark.asyncio
class TestProtectedRoute:
    async def test_me_returns_current_user(self, client: AsyncClient):
        await _register(client)
        tokens = await _login(client)

        resp = await client.get(
            f"{_BASE}/auth/me",
            headers={"Authorization": f"Bearer {tokens['accessToken']}"},
        )
        assert resp.status_code == 200
        assert resp.json()["email"] == _VALID_USER["email"]

    async def test_me_without_token_returns_401(self, client: AsyncClient):
        resp = await client.get(f"{_BASE}/auth/me")
        assert resp.status_code == 401

    async def test_me_with_expired_token_returns_401(self, client: AsyncClient):
        from datetime import timedelta
        from jose import jwt as jose_jwt
        from src.core.config import get_settings

        settings = get_settings()
        past = __import__("datetime").datetime.now(tz=__import__("datetime").timezone.utc) - timedelta(seconds=1)
        expired = jose_jwt.encode(
            {"sub": str(__import__("uuid").uuid4()), "type": "access", "exp": past},
            settings.JWT_SECRET_KEY,
            algorithm=settings.JWT_ALGORITHM,
        )
        resp = await client.get(
            f"{_BASE}/auth/me",
            headers={"Authorization": f"Bearer {expired}"},
        )
        assert resp.status_code == 401

    async def test_me_with_refresh_token_returns_401(self, client: AsyncClient):
        """A refresh JWT must be rejected on an access-token-protected route."""
        await _register(client)
        tokens = await _login(client)

        resp = await client.get(
            f"{_BASE}/auth/me",
            headers={"Authorization": f"Bearer {tokens['refreshToken']}"},
        )
        assert resp.status_code == 401

    async def test_me_with_malformed_header_returns_401(self, client: AsyncClient):
        resp = await client.get(
            f"{_BASE}/auth/me",
            headers={"Authorization": "NotBearer token"},
        )
        assert resp.status_code == 401