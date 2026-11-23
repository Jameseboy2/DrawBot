#pragma once

#include "BotState.h"
#include "NetworkManager.h"

class StateManager {
public:
    StateManager();

    [[nodiscard]] BotState get_state() const;

    [[nodiscard]] const String &get_last_poll_response() const;

    void set_state(BotState bot_state);

    void poll_server_if_due();

    void poll_server();

private:
    BotState m_bot_state;
    String m_last_poll_response;
    NetworkManager m_network_manager;
    unsigned long m_last_poll_time;
};
