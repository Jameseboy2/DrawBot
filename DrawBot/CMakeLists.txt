# Used for CLion code highlighting/tooltips only.

cmake_minimum_required(VERSION 3.20)
project(DrawBot CXX)

set(CMAKE_CXX_STANDARD 17)

# Dynamically locate the Arduino core headers installed by arduino-cli
set(USER_HOME $ENV{HOME})

include_directories(
        "${USER_HOME}/.arduino15/packages/arduino/hardware/avr/1.8.8/cores/arduino"
        "${USER_HOME}/.arduino15/packages/arduino/hardware/avr/1.8.8/variants/standard"
)

# Tell CLion to index all source and header files in your project
file(GLOB_RECURSE SOURCES
        "*.ino"
        "*.cpp"
        "*.h"
        "src/*.cpp"
        "src/*.h"
)

add_executable(DrawBot ${SOURCES})
