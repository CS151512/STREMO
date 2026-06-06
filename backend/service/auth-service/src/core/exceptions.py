"""Custom exceptions for auth service."""


class AuthError(Exception):
    """Base exception for authentication errors."""
    pass


class InvalidCredentialsError(AuthError):
    """Raised when credentials are invalid."""
    pass


class UserNotFoundError(AuthError):
    """Raised when user is not found."""
    pass


class UserAlreadyExistsError(AuthError):
    """Raised when trying to create a user that already exists."""
    pass


class TokenExpiredError(AuthError):
    """Raised when token has expired."""
    pass


class InvalidTokenError(AuthError):
    """Raised when token is invalid."""
    pass


class TokenNotFoundError(AuthError):
    """Raised when token is not found in storage."""
    pass


class PermissionDeniedError(AuthError):
    """Raised when user lacks required permissions."""
    pass


class RateLimitExceededError(AuthError):
    """Raised when rate limit is exceeded."""
    pass


class DatabaseError(AuthError):
    """Raised when database operation fails."""
    pass


class CacheError(AuthError):
    """Raised when cache operation fails."""
    pass