#include "models/domain_types.h"
#include <random>
#include <sstream>
#include <iomanip>
#include <regex>
#include <stdexcept>
#include <algorithm>

namespace auth {
    namespace models {
        // RefreshToken implementation
        RefreshToken RefreshToken::generate() {
            const std::string chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
            std::random_device rd;
            std::mt19937 gen(rd());
            std::uniform_int_distribution<> dis(0, chars.size() - 1);

            std::string token;
            token.reserve(64);
            for (int i = 0; i < 64; ++i) {
                token += chars[dis(gen)];
            }
            return RefreshToken(token);
        }

        RefreshToken RefreshToken::fromString(const std::string &token) {
            return RefreshToken(token);
        }

        RefreshToken::RefreshToken(const std::string &token) : token_(token) {
        }

        bool RefreshToken::isValid() const {
            return token_.length() == 64 &&
                   std::all_of(token_.begin(), token_.end(), [](char c) {
                       return std::isalnum(static_cast<unsigned char>(c));
                   });
        }

        // Email implementation
        bool Email::validate(const std::string &email) {
            const std::regex pattern(R"((\w+)(\.\w+)*@(\w+)(\.\w+)+)");
            return std::regex_match(email, pattern) && email.length() <= 255;
        }

        Email Email::fromString(const std::string &email) {
            if (!validate(email)) {
                throw std::invalid_argument("Invalid email format");
            }
            return Email(email);
        }

        Email::Email(const std::string &email) : email_(email) {
        }

        bool Email::isValid() const {
            return validate(email_);
        }

        // Username implementation
        bool Username::validate(const std::string &username) {
            const std::regex pattern(R"([a-zA-Z0-9_]{3,32})");
            return std::regex_match(username, pattern);
        }

        Username Username::fromString(const std::string &username) {
            if (!validate(username)) {
                throw std::invalid_argument("Username must be 3-32 characters (letters, numbers, underscore)");
            }
            return Username(username);
        }

        Username::Username(const std::string &username) : username_(username) {
        }

        bool Username::isValid() const {
            return validate(username_);
        }

        // Password implementation
        bool Password::validateStrength(const std::string &password) {
            if (password.length() < 8) return false;

            bool has_upper = false;
            bool has_lower = false;
            bool has_digit = false;
            bool has_special = false;

            for (char c: password) {
                if (std::isupper(static_cast<unsigned char>(c))) has_upper = true;
                else if (std::islower(static_cast<unsigned char>(c))) has_lower = true;
                else if (std::isdigit(static_cast<unsigned char>(c))) has_digit = true;
                else has_special = true;
            }

            return has_upper && has_lower && has_digit && has_special;
        }

        Password Password::fromString(const std::string &password) {
            if (!validateStrength(password)) {
                throw std::invalid_argument(
                    "Password must be at least 8 characters with uppercase, lowercase, digit and special character");
            }
            return Password(password);
        }

        Password::Password(const std::string &password) : password_(password) {
        }

        bool Password::isStrong() const {
            return validateStrength(password_);
        }
    } // namespace models
} // namespace auth
