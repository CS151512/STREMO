#pragma once

#include "domain.h"
#include "domain_types.h"
#include <string>
#include <optional>
#include <chrono>

namespace auth {
    namespace models {
        // Запросы
        struct RegisterRequest {
            std::string email;
            std::string username;
            std::string password;
            std::string confirm_password;
        };

        struct LoginRequest {
            std::string login; // email / username
            std::string password;
            std::string user_agent;
            std::string ip_address;
        };

        struct RefreshTokenRequest {
            std::string refresh_token;
        };

        struct LogoutRequest {
            std::string access_token;
            std::string refresh_token;
        };

        struct ChangePasswordRequest {
            std::string current_password;
            std::string new_password;
            std::string confirm_new_password;
        };

        struct UpdateProfileRequest {
            std::optional<std::string> email;
            std::optional<std::string> username;
        };

        // Responses
        struct AuthResponse {
            std::string access_token;
            std::string refresh_token;
            std::string token_type = "Bearer";
            int64_t expires_in;
            User user_info;
        };

        struct UserInfoResponse {
            UserId user_id;
            std::string email;
            std::string username;
            UserRole role;
            UserStatus status;
            std::chrono::system_clock::time_point created_at;
            std::optional<std::chrono::system_clock::time_point> last_login_at;
        };

        struct SessionInfoResponse {
            SessionId session_id;
            std::string user_agent;
            std::string ip_address;
            std::chrono::system_clock::time_point created_at;
            std::chrono::system_clock::time_point expires_at;
            bool is_current;
        };

        struct ErrorResponse {
            int code;
            std::string message;
            std::optional<std::string> details;
        };

        // JWT Claims
        struct JWTClaims {
            UserId user_id;
            std::string username;
            UserRole role;
            SessionId session_id;
            std::chrono::system_clock::time_point issued_at;
            std::chrono::system_clock::time_point expires_at;

            std::string toJson() const;

            static JWTClaims fromJson(const std::string &json);
        };

        // Вспомогательный DTO для создания пользователя в БД
        struct CreateUserDTO {
            std::string email;
            std::string username;
            std::string password_hash;
            UserRole role = UserRole::VIEWER;
            UserStatus status = UserStatus::INACTIVE;
        };

        // DTO для обновления пользователя
        struct UpdateUserDTO {
            std::optional<std::string> email;
            std::optional<std::string> username;
            std::optional<UserRole> role;
            std::optional<UserStatus> status;
            std::optional<std::chrono::system_clock::time_point> last_login_at;
        };

        // DTO для создания сессии
        struct CreateSessionDTO {
            UserId user_id;
            std::string refresh_token;
            std::string user_agent;
            std::string ip_address;
            std::chrono::system_clock::time_point expires_at;
        };
    } // namespace models
} // namespace auth
