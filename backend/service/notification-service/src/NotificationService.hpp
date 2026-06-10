#pragma once

#include <condition_variable>
#include <memory>
#include <mutex>
#include <string>
#include <thread>
#include <vector>

#include <librdkafka/rdkafkacpp.h>
#include "httplib.h"

#include "Config.hpp"

namespace stremo::notification {

class NotificationService {
public:
    explicit NotificationService(const Config& cfg);
    ~NotificationService();

    NotificationService(const NotificationService&) = delete;
    NotificationService& operator=(const NotificationService&) = delete;

    void Start();
    void Stop();

private:
    void ConsumeKafkaLoop();
    void Broadcast(const std::string& message);
    void SetupRoutes();

    Config config_;
    httplib::Server http_server_;

    std::thread http_thread_;
    std::thread kafka_thread_;
    bool running_;

    struct ClientSession {
        std::vector<std::string> message_queue;
    };

    std::mutex clients_mutex_;
    std::vector<std::shared_ptr<ClientSession>> sessions_;
    std::condition_variable cv_;
};

}
