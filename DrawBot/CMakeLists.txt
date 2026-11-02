# Used for CLion code highlighting/tooltips only.

cmake_minimum_required(VERSION 3.20)
project(DrawBot CXX)

set(CMAKE_CXX_STANDARD 17)

# Dynamically locate the Arduino core headers installed by arduino-cli
set(USER_HOME $ENV{HOME})

include_directories(
        "${USER_HOME}/.arduino15/packages/esp8266/hardware/esp8266/3.1.2/cores/esp8266"
        "${USER_HOME}/.arduino15/packages/esp8266/hardware/esp8266/3.1.2/variants/nodemcu"
        "${USER_HOME}/.arduino15/packages/esp8266/hardware/esp8266/3.1.2/libraries/ESP8266WiFi/src"
        "${USER_HOME}/.arduino15/packages/esp8266/hardware/esp8266/3.1.2/libraries/ESP8266HTTPClient/src"
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
