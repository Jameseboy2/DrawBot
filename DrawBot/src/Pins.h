#pragma once
#include <Arduino.h>

// Note: Caution with using D4 and D8.

class Pins {
public:
    static constexpr int EN_1 = D5;
    static constexpr int DIR_1 = D6;
    static constexpr int PUL_1 = D7;

    static constexpr int EN_2 = D0;
    static constexpr int DIR_2 = D1;
    static constexpr int PUL_2 = D2;

    // The bot can be put into "ready" or "not ready" mode, and puts itself into "not ready" mode sometimes.
    // When in "not ready" mode, the bot will reset itself and otherwise not move until put into "ready" mode.
    // The button toggles "ready" mode and the LED is on when the bot is "ready".
    static constexpr int READY_BUTTON = D3;
    static constexpr int READY_LED = LED_BUILTIN;
};
