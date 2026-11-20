#pragma once

#include <ESP8266HTTPClient.h>

#include "BotState.h"

class NetworkManager {
public:
    NetworkManager();

    static void connect();

    void post_bot_poll(BotState bot_state, String &response);

private:
    WiFiClient m_client;
    HTTPClient m_http;
};
