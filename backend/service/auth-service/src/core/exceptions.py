from typing import Any, Optional


class AuthServiceException(Exception):
    """Base exception for auth service"""

    def __init__(
            self,
            message: str = "An error occurred",
            status_code: int = 500,
            details: Optional[dict[str, Any]] = None,
    ):
        self.message = message
        self.status_code = status_code
        self.details = details or {}
        super().__init__(self.message)


class DuplicateError(AuthServiceException):
    """Raised when trying to create a duplicate resource"""

    def __init__(self, message: str = "Resource already exists", details: Optional[dict[str, Any]] = None):
        super().__init__(message, status_code=409, details=details)


class NotFoundError(AuthServiceException):
    """Raised when resource not found"""

    def __init__(self, message: str = "Resource not found", details: Optional[dict[str, Any]] = None):
        super().__init__(message, status_code=404, details=details)


class ValidationError(AuthServiceException):
    """Raised when validation fails"""

    def __init__(self, message: str = "Validation error", details: Optional[dict[str, Any]] = None):
        super().__init__(message, status_code=400, details=details)


class UnauthorizedError(AuthServiceException):
    """Raised when authentication fails"""

    def __init__(self, message: str = "Unauthorized", details: Optional[dict[str, Any]] = None):
        super().__init__(message, status_code=401, details=details)


class ForbiddenError(AuthServiceException):
    """Raised when user lacks permissions"""

    def __init__(self, message: str = "Forbidden", details: Optional[dict[str, Any]] = None):
        super().__init__(message, status_code=403, details=details)


class TokenExpiredError(UnauthorizedError):
    """Raised when token has expired"""

    def __init__(self, message: str = "Token has expired", details: Optional[dict[str, Any]] = None):
        super().__init__(message, details)


class InvalidTokenError(UnauthorizedError):
    """Raised when token is invalid"""

    def __init__(self, message: str = "Invalid token", details: Optional[dict[str, Any]] = None):
        super().__init__(message, details)
