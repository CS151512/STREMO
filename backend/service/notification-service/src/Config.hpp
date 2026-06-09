#pragma once

#include <cstdlib>
#include <string>

namespace stremo::notification {

struct Config {
    std::string port;
    std::string kafka_brokers;
    std::string topic;

    static Config Load() {
        Config cfg;

        if (const char* env_p = std::getenv("PORT")) {
            cfg.port = env_p;
        } else {
            cfg.port = "8087";
        }

        if (const char* env_b = std::getenv("KAFKA_BROKERS")) {
            cfg.kafka_brokers = env_b;
        } else {
            cfg.kafka_brokers = "localhost:19092";
        }

        if (const char* env_t = std::getenv("KAFKA_TOPIC")) {
            cfg.topic = env_t;
        } else {
            cfg.topic = "stream.alerts";
        }

        return cfg;
    }
};

}
