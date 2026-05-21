cmake_minimum_required(VERSION 3.20)
project(auth_service)

add_executable(auth_service src/main.cpp)
target_link_libraries(auth_service PRIVATE cpp_core)
