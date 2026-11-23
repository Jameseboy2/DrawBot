#include "StateManager.h"

#include <Arduino.h>

#include "../Pins.h"

StateManager::StateManager() : m_bot_state(BotState::AwaitingReadyButton), m_last_poll_time(millis()) {
    NetworkManager::connect();
}

/// Sets the bot state, automatically updating the LED and the remote server.
void StateManager::set_state(const BotState bot_state) {
    m_bot_state = bot_state;

    if (m_bot_state == BotState::AwaitingReadyButton) {
        digitalWrite(Pins::READY_LED, HIGH); // off
    } else {
        digitalWrite(Pins::READY_LED, LOW); // on
    }

    poll_server();
}

/// Polls the server if it's been long enough since the last time we did it that it's due for an update.
void StateManager::poll_server_if_due() {
    unsigned long poll_interval_ms = 5000;
    if (m_bot_state == BotState::Working) {
        poll_interval_ms = 30000;
    }

    if (millis() - m_last_poll_time > poll_interval_ms) {
        poll_server();
    }
}

void StateManager::poll_server() {
    m_last_poll_time = millis();
    // TODO
    // m_network_manager.post_bot_poll('', nullptr);
}
