#pragma once

#include "models/dto.h"
#include "models/domain_types.h"
#include <string>
#include <optional>
#include <vector>

namespace auth {
    namespace service {
        // Validator class
        class Validator {
        public:
            Validator() = default;

            ~Validator() = default;

            // Register Request validation
            void validateRegisterRequest(const models::RegisterRequest &request);

            // Login Request validation
            void validateLoginRequest(const models::LoginRequest &request);

            // Password Change validation
            void validateChangePasswordRequest(const models::ChangePasswordRequest &request);

            // Profile Update validation
            void validateUpdateProfileRequest(const models::UpdateProfileRequest &request);

            // User ID validation
            void validateUserId(models::UserId user_id);

            // Token validation
            void validateToken(const std::string &token);

            // Refresh Token validation
            void validateRefreshToken(const std::string &refresh_token);

            // Email validation
            bool isValidEmail(const std::string &email);

            // Username validation
            bool isValidUsername(const std::string &username);

            // Password validation
            bool isValidPassword(const std::string &password);

            // Check if passwords equal
            void validatePasswordsMatch(const std::string &password, const std::string &confirm_password);

            // Deleting dangerous symbols
            std::string sanitizeInput(const std::string &input);

            // IP address validating
            bool isValidIpAddress(const std::string &ip);

            // User Agent validating
            bool isValidUserAgent(const std::string &user_agent);

        private:
            // Regexes for validation
            static const std::string EMAIL_REGEX_PATTERN;
            static const std::string USERNAME_REGEX_PATTERN;
            static const std::string IPV4_REGEX_PATTERN;
            static const std::string IPV6_REGEX_PATTERN;

            // Utility methods
            void throwValidationError(const std::string &field, const std::string &message);

            bool isAsciiPrintable(const std::string &str);
        };
    } // namespace service
} // namespace auth
