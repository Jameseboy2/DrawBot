#include "main.h"

#include "Pins.h"
#include "position/PositionManager.h"
#include "state/StateManager.h"

volatile bool ready_button_pressed = false;

void main_setup() {
    pinMode(Pins::EN_1, OUTPUT);
    pinMode(Pins::DIR_1, OUTPUT);
    pinMode(Pins::PUL_1, OUTPUT);
    pinMode(Pins::EN_2, OUTPUT);
    pinMode(Pins::DIR_2, OUTPUT);
    pinMode(Pins::PUL_2, OUTPUT);

    pinMode(Pins::READY_BUTTON, INPUT_PULLUP);
    pinMode(Pins::READY_LED, OUTPUT);
    attachInterrupt(digitalPinToInterrupt(Pins::READY_BUTTON), handle_ready_button, RISING);

    // The drivers take at least 200ms to process the enable commands, but because we're waiting to connect to
    // the network and for the button to be pressed before doing anything, that won't be a problem.
    digitalWrite(Pins::EN_1, LOW);
    digitalWrite(Pins::EN_2, LOW);

    Serial.begin(115200);
}

void main_loop() {
    static auto position_manager = PositionManager();
    static auto state_manager = StateManager();
    static auto last_unsigned_button_press = millis();

    // If the ready button has been pressed since the last loop (debounced), update the bot state.
    if (ready_button_pressed) {
        ready_button_pressed = false;

        const auto now = millis();
        if (now - last_unsigned_button_press > 50) {
            last_unsigned_button_press = now;

            if (state_manager.get_state() == AWAITING_READY_BUTTON) {
                state_manager.set_state(READY_FOR_JOB);
            } else {
                // TODO: return to the home position
                state_manager.set_state(AWAITING_READY_BUTTON);
            }
        }
    }

    if (
        state_manager.get_state() == READY_FOR_JOB
        && state_manager.get_last_poll_response() != ""
        && state_manager.get_last_poll_response() != "NO_PENDING_JOB"
    ) {
        state_manager.set_state(WORKING);

        // TODO: Decode the command string (last_poll_response). Or, ideally, parse directly from the stream
        //  while reading the response to save on RAM.
        //  Test with some big/detailed images - even with stream parsing, we may need to implement
        //  command pagination if the commands are just too big.
    }

    if (state_manager.get_state() == WORKING) {
        // TODO: Run one line of the command string with position_manager.
        //  (only do one line here so we can be relatively responsive for polling and button handling)
        //  If there are no more lines to run, return to the home position and set bot_state to AWAITING_READY_BUTTON.
    }

    // Update the server with our status every so often.
    state_manager.poll_server_if_due();

    delay(1);
}

void handle_ready_button() {
    ready_button_pressed = true;
}
