import re

from src.core.exceptions import WeakPasswordError

# At least: 1 uppercase, 1 lowercase, 1 digit, 1 special char
_PASSWORD_RE = re.compile(
    r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*()_+\-=\[\]{};':\"\\|,.<>\/?]).{8,128}$"
)

_COMMON_PASSWORDS = {
    "Password1!",
    "Password123!",
    "Qwerty123!",
    "Welcome1!",
    "Admin123!",
}


def validate_password_strength(password: str) -> None:
    """
    Enforce password strength rules.
    Rules:
    - 8–128 characters
    - At least one uppercase letter
    - At least one lowercase letter
    - At least one digit
    - At least one special character
    - Not a commonly used password
    Raises:
        WeakPasswordError: when any rule is violated.
    """
    if password in _COMMON_PASSWORDS:
        raise WeakPasswordError("This password is too common.")

    if not _PASSWORD_RE.match(password):
        raise WeakPasswordError(
            "Password must be 8–128 characters and contain at least one uppercase letter, "
            "one lowercase letter, one digit, and one special character."
        )


def validate_email_domain(email: str, blocked_domains: set[str] | None = None) -> None:
    """
    Optionally reject sign-ups from disposable / blocked email domains.
    Raises:
        ValidationException: when the email domain is blocked.
    """
    if not blocked_domains:
        return

    from src.core.exceptions import ValidationException

    domain = email.split("@")[-1].lower()
    if domain in blocked_domains:
        raise ValidationException(f"Sign-ups from '{domain}' are not allowed.")
