#include "NotificationService.hpp"

#include <iostream>
#include <nlohmann/json.hpp>

namespace stremo::notification {

NotificationService::NotificationService(const Config& cfg)
    : config_(cfg), running_(false) {
    SetupRoutes();
}

NotificationService::~NotificationService() {
    Stop();
}

void NotificationService::SetupRoutes() {
    http_server_.Get("/api/v1/notifications/stream",
        [this](const httplib::Request& /*req*/, httplib::Response& res) {
            res.set_header("Content-Type", "text/event-stream");
            res.set_header("Cache-Control", "no-cache");
            res.set_header("Connection", "keep-alive");
            res.set_header("Access-Control-Allow-Origin", "*");

            auto session = std::make_shared<ClientSession>();
            {
                std::lock_guard<std::mutex> lock(clients_mutex_);
                sessions_.push_back(session);
            }

            std::cout << "[HTTP] New SSE client connected." << std::endl;

            res.set_chunked_content_provider(
                "text/event-stream",
                [this, session](size_t /*offset*/, httplib::DataSink& sink) {
                    std::unique_lock<std::mutex> lock(clients_mutex_);

                    cv_.wait(lock, [this, session] {
                        return !running_ || !session->message_queue.empty();
                    });

                    if (!running_) {
                        return false;
                    }

                    for (const auto& msg : session->message_queue) {
                        std::string sse_msg = "data: " + msg + "\n\n";
                        sink.write(sse_msg.c_str(), sse_msg.size());
                    }
                    session->message_queue.clear();

                    return true;
                }
            );
        }
    );
}

void NotificationService::Broadcast(const std::string& message) {
    std::lock_guard<std::mutex> lock(clients_mutex_);
    for (auto& session : sessions_) {
        session->message_queue.push_back(message);
    }
    cv_.notify_all();
}

void NotificationService::ConsumeKafkaLoop() {
    std::string errstr;

    RdKafka::Conf* conf = RdKafka::Conf::create(RdKafka::Conf::CONF_GLOBAL);
    conf->set("metadata.broker.list", config_.kafka_brokers, errstr);
    conf->set("group.id", "notification-group", errstr);
    conf->set("auto.offset.reset", "latest", errstr);

    RdKafka::Consumer* consumer = RdKafka::Consumer::create(conf, errstr);
    if (!consumer) {
        std::cerr << "[Kafka] Failed to create consumer: " << errstr << std::endl;
        delete conf;
        return;
    }
    delete conf;

    RdKafka::Topic* topic = RdKafka::Topic::create(consumer, config_.topic, nullptr, errstr);
    if (!topic) {
        std::cerr << "[Kafka] Failed to create topic: " << errstr << std::endl;
        delete consumer;
        return;
    }

    consumer->start(topic, 0, RdKafka::Topic::OFFSET_END);
    std::cout << "[Kafka] Listening to topic: " << config_.topic << std::endl;

    while (running_) {
        RdKafka::Message* msg = consumer->consume(topic, 0, 1000);

        if (msg->err() == RdKafka::ERR_NO_ERROR) {
            std::string payload(static_cast<const char*>(msg->payload()), msg->len());
            std::cout << "[Kafka] Received alert: " << payload << std::endl;
            Broadcast(payload);
        }

        delete msg;
    }

    consumer->stop(topic, 0);
    consumer->poll(1000);

    delete topic;
    delete consumer;
}

void NotificationService::Start() {
    running_ = true;

    kafka_thread_ = std::thread(&NotificationService::ConsumeKafkaLoop, this);

    http_thread_ = std::thread([this]() {
        int port = std::stoi(config_.port);
        std::cout << "[HTTP] SSE Server listening on port " << port << std::endl;
        http_server_.listen("0.0.0.0", port);
    });
}

void NotificationService::Stop() {
    if (running_) {
        running_ = false;
        http_server_.stop();
        cv_.notify_all();

        if (http_thread_.joinable()) {
            http_thread_.join();
        }

        if (kafka_thread_.joinable()) {
            kafka_thread_.join();
        }

        std::cout << "[System] Service stopped gracefully." << std::endl;
    }
}

}
