#include "models/dto.h"
#include <nlohmann/json.hpp>

namespace auth {
    namespace models {
        std::string JWTClaims::toJson() const {
            nlohmann::json j;
            j["user_id"] = user_id.value;
            j["username"] = username;
            j["role"] = static_cast<uint8_t>(role);
            j["session_id"] = session_id.value;
            j["issued_at"] = std::chrono::duration_cast<std::chrono::seconds>(
                issued_at.time_since_epoch()).count();
            j["expires_at"] = std::chrono::duration_cast<std::chrono::seconds>(
                expires_at.time_since_epoch()).count();
            return j.dump();
        }

        JWTClaims JWTClaims::fromJson(const std::string &json) {
            auto j = nlohmann::json::parse(json);
            JWTClaims claims;
            claims.user_id = UserId(j.at("user_id").get<uint64_t>());
            claims.username = j.at("username").get<std::string>();
            claims.role = static_cast<UserRole>(j.at("role").get<uint8_t>());
            claims.session_id = SessionId(j.at("session_id").get<std::string>());

            auto issued = std::chrono::seconds(j.at("issued_at").get<int64_t>());
            claims.issued_at = std::chrono::system_clock::time_point(issued);

            auto expires = std::chrono::seconds(j.at("expires_at").get<int64_t>());
            claims.expires_at = std::chrono::system_clock::time_point(expires);

            return claims;
        }
    } // namespace models
} // namespace auth
