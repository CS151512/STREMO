#[derive(Debug)]
pub enum ErrorCode {
    // Auth Errors
    InvalidCredentials,
    TokenExpired,
    AccountBanned,

    // Billing Errors
    InsufficientFunds,
    AmountTooLow,

    // Stream & Chat
    ChannelNotFound,
    NotStreamOwner,
    RateLimitExceeded,

    // System
    InternalServerError,
    ValidationFailed,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::InvalidCredentials => "INVALID_CREDENTIALS",
            ErrorCode::TokenExpired => "TOKEN_EXPIRED",
            ErrorCode::AccountBanned => "ACCOUNT_BANNED",
            ErrorCode::InsufficientFunds => "INSUFFICIENT_FUNDS",
            ErrorCode::AmountTooLow => "AMOUNT_TOO_LOW",
            ErrorCode::ChannelNotFound => "CHANNEL_NOT_FOUND",
            ErrorCode::NotStreamOwner => "NOT_STREAM_OWNER",
            ErrorCode::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            ErrorCode::InternalServerError => "INTERNAL_SERVER_ERROR",
            ErrorCode::ValidationFailed => "VALIDATION_FAILED",
        }
    }
}
