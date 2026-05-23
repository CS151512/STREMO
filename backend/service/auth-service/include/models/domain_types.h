#pragma once

#include <string>
#include <cstdint>

namespace auth {
    namespace models {
        // Strong typedefs для избежания путаницы ID
        struct UserId {
            uint64_t value;

            explicit UserId(uint64_t val = 0) : value(val) {
            }

            operator uint64_t() const { return value; }

            bool operator==(const UserId &other) const { return value == other.value; }
            bool operator!=(const UserId &other) const { return value != other.value; }
        };

        struct SessionId {
            std::string value;

            explicit SessionId(const std::string &val = "") : value(val) {
            }

            operator std::string() const { return value; }

            bool operator==(const SessionId &other) const { return value == other.value; }
            bool operator!=(const SessionId &other) const { return value != other.value; }
        };

        // Enum для ролей пользователя
        enum class UserRole : uint8_t {
            VIEWER = 0, // Обычный зритель
            STREAMER = 1, // Стpимер
            MODERATOR = 2, // Модератор
            ADMIN = 3 // Администратор
        };

        // Enum для статуса пользователя
        enum class UserStatus : uint8_t {
            ACTIVE = 0, // Активен
            BANNED = 1, // Забанен
            DELETED = 2, // Удален
            INACTIVE = 3, // Не активен (подтверждение email)
            SUSPENDED = 4 // Временно заблокирован
        };

        // Refresh токен со вспомогательными методами
        class RefreshToken {
        public:
            static RefreshToken generate();

            static RefreshToken fromString(const std::string &token);

            const std::string &toString() const { return token_; }

            bool isValid() const;

        private:
            RefreshToken() = default;

            explicit RefreshToken(const std::string &token);

            std::string token_;
        };

        // Email с валидацией
        class Email {
        public:
            static bool validate(const std::string &email);

            static Email fromString(const std::string &email);

            const std::string &toString() const { return email_; }

            bool isValid() const;

        private:
            explicit Email(const std::string &email);

            std::string email_;
        };

        // Username с валидацией
        class Username {
        public:
            static bool validate(const std::string &username);

            static Username fromString(const std::string &username);

            const std::string &toString() const { return username_; }

            bool isValid() const;

        private:
            explicit Username(const std::string &username);

            std::string username_;
        };

        // Пароль с валидацией (не храним в открытом виде, только для проверки силы)
        class Password {
        public:
            static bool validateStrength(const std::string &password);

            static Password fromString(const std::string &password);

            const std::string &toString() const { return password_; }

            bool isStrong() const;

        private:
            explicit Password(const std::string &password);

            std::string password_;
        };
    } // namespace models
} // namespace auth
