#include "main.h"

#include "Pins.h"
#include "position/PositionManager.h"
#include "state/StateManager.h"

static volatile bool ready_button_pressed = false;

void main_setup() {
    Serial.begin(115200);

    pinMode(Pins::DIR_1, OUTPUT);
    pinMode(Pins::DIR_2, OUTPUT);
    pinMode(Pins::PUL_1, OUTPUT);
    pinMode(Pins::PUL_2, OUTPUT);
    pinMode(Pins::EN_1, OUTPUT);
    pinMode(Pins::EN_2, OUTPUT);

    pinMode(Pins::READY_BUTTON, INPUT_PULLUP);
    attachInterrupt(digitalPinToInterrupt(Pins::READY_BUTTON), handle_ready_button, FALLING);

    pinMode(Pins::READY_LED, OUTPUT);
    digitalWrite(Pins::READY_LED, HIGH); // off

    // The drivers take at least 200ms to process the enable commands, but because we're waiting to connect to
    // the network and for the button to be pressed before doing anything, that won't be a problem.
    digitalWrite(Pins::EN_1, LOW);
    digitalWrite(Pins::EN_2, LOW);
}

void main_loop() {
    static auto position_manager = PositionManager();
    static auto state_manager = StateManager();
    static auto last_button_press = millis();

    // If the ready button has been pressed since the last loop (debounced), update the bot state.
    if (ready_button_pressed) {
        ready_button_pressed = false;

        const auto now = millis();
        if (now - last_button_press > 50) {
            last_button_press = now;

            if (state_manager.get_state() == BotState::AwaitingReadyButton) {
                state_manager.set_state(BotState::ReadyForJob);
            } else {
                // TODO: return to the home position, and don't update the state until home
                state_manager.set_state(BotState::AwaitingReadyButton);
            }
        }
    }

    if (
        state_manager.get_state() == BotState::ReadyForJob
        // && state_manager.get_last_poll_response() != ""
        // && state_manager.get_last_poll_response() != "NO_PENDING_JOB"
    ) {
        state_manager.set_state(BotState::Working);

        // TODO: Decode the command string.
        //  Also, fix the poll response comparison: Make 100% sure jobs can't get re-triggered, and only run things
        //  that look like command strings instead of blacklisting everything that's not a command string.
    }

    if (state_manager.get_state() == BotState::Working) {
        // TODO: Redesign this to use a timer interrupt for driving the stepper motors.
        //  (or, if that's too much work, just run one line of the command string with position_manager here -
        //  only do one line here so we can be relatively responsive for polling and button handling.)
        //  Swap command pages and request a new page whenever we reach the end of the current page.
        //  When done, return to the home position and then (once home) set bot_state to AwaitingReadyButton.
    }

    // Update the server with our status every so often.
    state_manager.poll_server_if_due();

    delay(1);
}

void IRAM_ATTR handle_ready_button() {
    ready_button_pressed = true;
}
