from http import HTTPStatus


class AppException(Exception):
    """Base application exception."""

    status_code: int = HTTPStatus.INTERNAL_SERVER_ERROR
    detail: str = "An unexpected error occurred."

    def __init__(self, detail: str | None = None) -> None:
        self.detail = detail or self.__class__.detail
        super().__init__(self.detail)


# --- Auth ---

class AuthException(AppException):
    """Base auth exception."""


class InvalidCredentialsError(AuthException):
    status_code = HTTPStatus.UNAUTHORIZED
    detail = "Invalid email or password."


class InvalidTokenError(AuthException):
    status_code = HTTPStatus.UNAUTHORIZED
    detail = "Token is invalid or has expired."


class TokenExpiredError(AuthException):
    status_code = HTTPStatus.UNAUTHORIZED
    detail = "Token has expired."


class InsufficientPermissionsError(AuthException):
    status_code = HTTPStatus.FORBIDDEN
    detail = "You do not have permission to perform this action."


# --- User ---

class UserException(AppException):
    """Base user exception."""


class UserNotFoundError(UserException):
    status_code = HTTPStatus.NOT_FOUND
    detail = "User not found."


class UserAlreadyExistsError(UserException):
    status_code = HTTPStatus.CONFLICT
    detail = "A user with this email already exists."


class UserInactiveError(UserException):
    status_code = HTTPStatus.FORBIDDEN
    detail = "This user account is inactive."


# --- Validation ---

class ValidationException(AppException):
    status_code = HTTPStatus.UNPROCESSABLE_ENTITY
    detail = "Validation failed."


class WeakPasswordError(ValidationException):
    detail = "Password does not meet the minimum security requirements."