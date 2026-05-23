#include "service/validator.h"
#include "errors/exceptions.h"

#include <regex>
#include <cctype>
#include <algorithm>

namespace auth {
    namespace service {
        const std::string Validator::EMAIL_REGEX_PATTERN = R"((\w+)(\.\w+)*@(\w+)(\.\w+)+)";
        const std::string Validator::USERNAME_REGEX_PATTERN = R"([a-zA-Z0-9_]{3,32})";
        const std::string Validator::IPV4_REGEX_PATTERN = R"((\d{1,3}\.){3}\d{1,3})";
        const std::string Validator::IPV6_REGEX_PATTERN = R"(([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4})";

        void Validator::validateRegisterRequest(const models::RegisterRequest &request) {
            if (!isValidEmail(request.email)) {
                throwValidationError("email", "Invalid email format");
            }

            if (!isValidUsername(request.username)) {
                throwValidationError("username", "Username must be 3-32 characters (letters, numbers, underscore)");
            }

            if (!isValidPassword(request.password)) {
                throwValidationError(
                    "password",
                    "Password must be at least 8 characters with uppercase, lowercase, digit and special character");
            }

            validatePasswordsMatch(request.password, request.confirm_password);
        }

        void Validator::validateLoginRequest(const models::LoginRequest &request) {
            if (request.login.empty()) {
                throwValidationError("login", "Login cannot be empty");
            }

            if (request.password.empty()) {
                throwValidationError("password", "Password cannot be empty");
            }

            if (!isValidUserAgent(request.user_agent)) {
                throwValidationError("user_agent", "Invalid user agent");
            }

            if (!isValidIpAddress(request.ip_address)) {
                throwValidationError("ip_address", "Invalid IP address");
            }
        }

        void Validator::validateChangePasswordRequest(const models::ChangePasswordRequest &request) {
            if (request.current_password.empty()) {
                throwValidationError("current_password", "Current password cannot be empty");
            }

            if (!isValidPassword(request.new_password)) {
                throwValidationError("new_password",
                                     "Password must be at least 8 characters with uppercase, lowercase, digit and special character");
            }

            validatePasswordsMatch(request.new_password, request.confirm_new_password);
        }

        void Validator::validateUpdateProfileRequest(const models::UpdateProfileRequest &request) {
            if (request.email.has_value() && !isValidEmail(request.email.value())) {
                throwValidationError("email", "Invalid email format");
            }

            if (request.username.has_value() && !isValidUsername(request.username.value())) {
                throwValidationError("username", "Username must be 3-32 characters (letters, numbers, underscore)");
            }
        }

        void Validator::validateUserId(models::UserId user_id) {
            if (user_id.value == 0) {
                throwValidationError("user_id", "Invalid user ID");
            }
        }

        void Validator::validateToken(const std::string &token) {
            if (token.empty()) {
                throwValidationError("token", "Token cannot be empty");
            }

            if (token.length() < 20) {
                // Минимальная длина JWT
                throwValidationError("token", "Token is too short");
            }
        }

        void Validator::validateRefreshToken(const std::string &refresh_token) {
            if (refresh_token.empty()) {
                throwValidationError("refresh_token", "Refresh token cannot be empty");
            }

            if (refresh_token.length() != 64) {
                throwValidationError("refresh_token", "Invalid refresh token length");
            }

            // Проверка, что токен состоит только из alphanumeric символов
            for (char c: refresh_token) {
                if (!std::isalnum(static_cast<unsigned char>(c))) {
                    throwValidationError("refresh_token", "Refresh token contains invalid characters");
                }
            }
        }

        bool Validator::isValidEmail(const std::string &email) {
            if (email.empty() || email.length() > 255) {
                return false;
            }

            std::regex pattern(EMAIL_REGEX_PATTERN);
            return std::regex_match(email, pattern);
        }

        bool Validator::isValidUsername(const std::string &username) {
            if (username.empty() || username.length() < 3 || username.length() > 32) {
                return false;
            }

            std::regex pattern(USERNAME_REGEX_PATTERN);
            return std::regex_match(username, pattern);
        }

        bool Validator::isValidPassword(const std::string &password) {
            if (password.length() < 8) {
                return false;
            }

            bool has_upper = false;
            bool has_lower = false;
            bool has_digit = false;
            bool has_special = false;

            for (char c: password) {
                if (std::isupper(static_cast<unsigned char>(c))) has_upper = true;
                else if (std::islower(static_cast<unsigned char>(c))) has_lower = true;
                else if (std::isdigit(static_cast<unsigned char>(c))) has_digit = true;
                else if (std::ispunct(static_cast<unsigned char>(c)) || std::isspace(static_cast<unsigned char>(c))) {
                    has_special = true;
                }
            }

            return has_upper && has_lower && has_digit && has_special;
        }

        void Validator::validatePasswordsMatch(const std::string &password, const std::string &confirm_password) {
            if (password != confirm_password) {
                throwValidationError("confirm_password", "Passwords do not match");
            }
        }

        std::string Validator::sanitizeInput(const std::string &input) {
            std::string sanitized;
            sanitized.reserve(input.length());

            for (char c: input) {
                // Разрешаем только ASCII печатные символы
                if (std::isprint(static_cast<unsigned char>(c)) && c != '<' && c != '>' && c != '&') {
                    sanitized += c;
                }
            }

            return sanitized;
        }

        bool Validator::isValidIpAddress(const std::string &ip) {
            if (ip.empty()) {
                return false;
            }

            // Проверка IPv4
            std::regex ipv4_pattern(IPV4_REGEX_PATTERN);
            if (std::regex_match(ip, ipv4_pattern)) {
                // Дополнительная проверка, что числа в диапазоне 0-255
                std::istringstream iss(ip);
                std::string octet;
                int octet_num = 0;

                while (std::getline(iss, octet, '.')) {
                    int value = std::stoi(octet);
                    if (value < 0 || value > 255) {
                        return false;
                    }
                    octet_num++;
                }

                return octet_num == 4;
            }

            // Проверка IPv6
            std::regex ipv6_pattern(IPV6_REGEX_PATTERN);
            return std::regex_match(ip, ipv6_pattern);
        }

        bool Validator::isValidUserAgent(const std::string &user_agent) {
            if (user_agent.empty() || user_agent.length() > 500) {
                return false;
            }

            // Проверка на опасные символы
            return isAsciiPrintable(user_agent);
        }

        bool Validator::isAsciiPrintable(const std::string &str) {
            return std::all_of(str.begin(), str.end(), [](char c) {
                return std::isprint(static_cast<unsigned char>(c));
            });
        }

        void Validator::throwValidationError(const std::string &field, const std::string &message) {
            throw errors::ValidationException("Validation failed for field '" + field + "': " + message);
        }
    } // namespace service
} // namespace auth
