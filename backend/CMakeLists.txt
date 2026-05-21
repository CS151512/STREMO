cmake_minimum_required(VERSION 3.20)
project(stremo_backend)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(libs/cpp-core)

add_subdirectory(service/auth-service)
add_subdirectory(service/notification-service)
add_subdirectory(service/smtp-service)
add_subdirectory(service/stream-meta-service)
add_subdirectory(service/user-profile-service)
add_subdirectory(service/vod-manager-service)
