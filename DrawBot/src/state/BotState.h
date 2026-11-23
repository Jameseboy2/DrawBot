#pragma once
#include <cstdint>

enum class BotState: uint8_t {
    AwaitingReadyButton,
    ReadyForJob,
    Working,
};
