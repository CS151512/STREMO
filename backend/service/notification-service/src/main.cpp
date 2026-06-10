#include <atomic>
#include <chrono>
#include <csignal>
#include <iostream>
#include <memory>
#include <thread>

#include "Config.hpp"
#include "NotificationService.hpp"

using namespace stremo::notification;

std::unique_ptr<NotificationService> g_service = nullptr;
std::atomic<bool> g_is_running{true};

void SignalHandler(int signal) {
    std::cout << "\n[System] Received signal " << signal
              << ", shutting down..." << std::endl;

    if (g_service) {
        g_service->Stop();
    }

    g_is_running = false;
}

int main() {
    std::signal(SIGINT, SignalHandler);
    std::signal(SIGTERM, SignalHandler);

    std::cout << "   STREMO Notification Service RUNN" << std::endl;

    try {
        Config cfg = Config::Load();
        g_service = std::make_unique<NotificationService>(cfg);

        g_service->Start();

        while (g_is_running) {
            std::this_thread::sleep_for(std::chrono::seconds(1));
        }
    } catch (const std::exception& ex) {
        std::cerr << "[Fatal Error] " << ex.what() << std::endl;
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
