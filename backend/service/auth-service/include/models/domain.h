#pragma once

#include "domain_types.h"
#include <string>
#include <chrono>
#include <optional>
#include <vector>

namespace auth {
    namespace models {
        // Основная сущность пользователя
        class User {
        public:
            User() = default;

            User(UserId id, std::string email, std::string username,
                 std::string password_hash, UserRole role, UserStatus status);

            // Getters
            UserId getId() const { return id_; }
            const std::string &getEmail() const { return email_; }
            const std::string &getUsername() const { return username_; }
            const std::string &getPasswordHash() const { return password_hash_; }
            UserRole getRole() const { return role_; }
            UserStatus getStatus() const { return status_; }
            const std::chrono::system_clock::time_point &getCreatedAt() const { return created_at_; }
            const std::chrono::system_clock::time_point &getUpdatedAt() const { return updated_at_; }

            const std::optional<std::chrono::system_clock::time_point> &getLastLoginAt() const {
                return last_login_at_;
            }

            // Setters (с валидацией)
            void setEmail(const std::string &email);

            void setUsername(const std::string &username);

            void setPasswordHash(const std::string &password_hash);

            void setRole(UserRole role);

            void setStatus(UserStatus status);

            void setLastLoginAt(const std::chrono::system_clock::time_point &time);

            void updateTimestamp();

            // Вспомогательные методы
            bool isActive() const;

            bool isBanned() const;

            bool isAdmin() const;

        private:
            UserId id_;
            std::string email_;
            std::string username_;
            std::string password_hash_;
            UserRole role_;
            UserStatus status_;
            std::chrono::system_clock::time_point created_at_;
            std::chrono::system_clock::time_point updated_at_;
            std::optional<std::chrono::system_clock::time_point> last_login_at_;
        };

        // Сущность сессии пользователя
        class Session {
        public:
            Session() = default;

            Session(SessionId id, UserId user_id, std::string refresh_token,
                    std::string user_agent, std::string ip_address,
                    std::chrono::system_clock::time_point expires_at);

            SessionId getId() const { return id_; }
            UserId getUserId() const { return user_id_; }
            const std::string &getRefreshToken() const { return refresh_token_; }
            const std::string &getUserAgent() const { return user_agent_; }
            const std::string &getIpAddress() const { return ip_address_; }
            const std::chrono::system_clock::time_point &getCreatedAt() const { return created_at_; }
            const std::chrono::system_clock::time_point &getExpiresAt() const { return expires_at_; }
            bool isRevoked() const { return is_revoked_; }

            void revoke();

            bool isExpired() const;

            void extendExpiration(const std::chrono::hours &hours);

        private:
            SessionId id_;
            UserId user_id_;
            std::string refresh_token_;
            std::string user_agent_;
            std::string ip_address_;
            std::chrono::system_clock::time_point created_at_;
            std::chrono::system_clock::time_point expires_at_;
            bool is_revoked_{false};
        };

        // Сущность для хранения в Redis (быстрый доступ)
        class UserCache {
        public:
            UserCache() = default;

            UserCache(UserId user_id, const std::string &email,
                      const std::string &username, UserRole role);

            UserId getUserId() const { return user_id_; }
            const std::string &getEmail() const { return email_; }
            const std::string &getUsername() const { return username_; }
            UserRole getRole() const { return role_; }
            const std::chrono::system_clock::time_point &getCachedAt() const { return cached_at_; }

            bool isCacheValid() const;

        private:
            UserId user_id_;
            std::string email_;
            std::string username_;
            UserRole role_;
            std::chrono::system_clock::time_point cached_at_;
        };
    } // namespace models
} // namespace auth
