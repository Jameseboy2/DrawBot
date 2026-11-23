#pragma once

#include <ESP8266HTTPClient.h>

#include "BotState.h"
#include "commands/CommandPage.h"

class NetworkManager {
public:
    NetworkManager();

    static void connect();

    void passive_poll_server(BotState bot_state, CommandPage *command_page);

    void request_command_page(int request_page_number, CommandPage &command_page);

private:
    void post_bot_poll(const char *payload, CommandPage *command_page);

    WiFiClient m_client;
    HTTPClient m_http;
};
