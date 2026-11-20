#include "StateManager.h"

#include <Arduino.h>

#include "../Pins.h"

StateManager::StateManager() : m_bot_state(AWAITING_READY_BUTTON), m_last_poll_time(millis()) {
    m_network_manager.connect();
}

BotState StateManager::get_state() const {
    return m_bot_state;
}

const String &StateManager::get_last_poll_response() const {
    return m_last_poll_response;
}

/// Sets the bot state, automatically updating the LED and the remote server.
void StateManager::set_state(const BotState bot_state) {
    m_bot_state = bot_state;

    if (m_bot_state == AWAITING_READY_BUTTON) {
        digitalWrite(Pins::READY_LED, LOW);
    } else {
        digitalWrite(Pins::READY_LED, HIGH);
    }

    poll_server();
}

/// Polls the server if it's been long enough since the last time we did it that it's due for an update.
void StateManager::poll_server_if_due() {
    unsigned long poll_interval_ms = 5000;
    if (m_bot_state == WORKING) {
        poll_interval_ms = 30000;
    }

    if (millis() - m_last_poll_time > poll_interval_ms) {
        poll_server();
    }
}

void StateManager::poll_server() {
    m_last_poll_time = millis();
    m_network_manager.post_bot_poll(m_bot_state, m_last_poll_response);
}
