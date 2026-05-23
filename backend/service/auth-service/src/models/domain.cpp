#include "models/domain.h"
#include <algorithm>
#include <regex>

namespace auth {
    namespace models {
        // User implementation
        User::User(UserId id, std::string email, std::string username,
                   std::string password_hash, UserRole role, UserStatus status)
            : id_(id)
              , email_(std::move(email))
              , username_(std::move(username))
              , password_hash_(std::move(password_hash))
              , role_(role)
              , status_(status)
              , created_at_(std::chrono::system_clock::now())
              , updated_at_(created_at_) {
        }

        void User::setEmail(const std::string &email) {
            if (!Email::validate(email)) {
                throw std::invalid_argument("Invalid email format");
            }
            email_ = email;
            updateTimestamp();
        }

        void User::setUsername(const std::string &username) {
            if (!Username::validate(username)) {
                throw std::invalid_argument("Invalid username format");
            }
            username_ = username;
            updateTimestamp();
        }

        void User::setPasswordHash(const std::string &password_hash) {
            if (password_hash.empty()) {
                throw std::invalid_argument("Password hash cannot be empty");
            }
            password_hash_ = password_hash;
            updateTimestamp();
        }

        void User::setRole(UserRole role) {
            role_ = role;
            updateTimestamp();
        }

        void User::setStatus(UserStatus status) {
            status_ = status;
            updateTimestamp();
        }

        void User::setLastLoginAt(const std::chrono::system_clock::time_point &time) {
            last_login_at_ = time;
            updateTimestamp();
        }

        void User::updateTimestamp() {
            updated_at_ = std::chrono::system_clock::now();
        }

        bool User::isActive() const {
            return status_ == UserStatus::ACTIVE;
        }

        bool User::isBanned() const {
            return status_ == UserStatus::BANNED;
        }

        bool User::isAdmin() const {
            return role_ == UserRole::ADMIN;
        }

        // Session implementation
        Session::Session(SessionId id, UserId user_id, std::string refresh_token,
                         std::string user_agent, std::string ip_address,
                         std::chrono::system_clock::time_point expires_at)
            : id_(std::move(id))
              , user_id_(user_id)
              , refresh_token_(std::move(refresh_token))
              , user_agent_(std::move(user_agent))
              , ip_address_(std::move(ip_address))
              , created_at_(std::chrono::system_clock::now())
              , expires_at_(expires_at) {
        }

        void Session::revoke() {
            is_revoked_ = true;
        }

        bool Session::isExpired() const {
            return std::chrono::system_clock::now() > expires_at_;
        }

        void Session::extendExpiration(const std::chrono::hours &hours) {
            expires_at_ = std::chrono::system_clock::now() + hours;
        }

        // UserCache implementation
        UserCache::UserCache(UserId user_id, const std::string &email,
                             const std::string &username, UserRole role)
            : user_id_(user_id)
              , email_(email)
              , username_(username)
              , role_(role)
              , cached_at_(std::chrono::system_clock::now()) {
        }

        bool UserCache::isCacheValid() const {
            auto now = std::chrono::system_clock::now();
            auto diff = std::chrono::duration_cast<std::chrono::hours>(now - cached_at_);
            return diff.count() < 1; // Кэш валиден 1 час
        }
    } // namespace models
} // namespace auth
