#include "NetworkManager.h"

#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>

#include "commands/CommandPage.h"
#include "../secrets.h"

NetworkManager::NetworkManager() {
    m_http.setReuse(true);
    // Disable chunked encoding.
    m_http.useHTTP10(true);
    m_http.setTimeout(2000);
}

void NetworkManager::connect() {
    // The credentials come from secrets.h, which is not committed to git.
    WiFi.begin(ssid, password);
    Serial.print("Connecting to network");
    while (WiFi.status() != WL_CONNECTED) {
        delay(200);
        Serial.print(".");
    }
    Serial.print("\nConnected.");
}

/// Posts to /bot-poll, providing the current bot status and not requesting anything in particular.
/// Parses into command_page, if a command page is received and command_page is provided.
void NetworkManager::passive_poll_server(const BotState bot_state, CommandPage *command_page) {
    const char *payload;
    switch (bot_state) {
        case BotState::AwaitingReadyButton:
            payload = R"({"status":"AwaitingReset"})";
        case BotState::ReadyForJob:
            payload = R"({"status":"Available"})";
        case BotState::Working:
            payload = R"({"status":"Working"})";
        default:
            payload = R"({"status":"Error"})";
    }

    post_bot_poll(payload, command_page);
}

/// Posts to /bot-poll, requesting a specific command page of the current image.
/// Passes the response to command_page.make_from_response if a page is received.
///
/// Should only be used when the bot state is Working. To get the first page (#0), use passive_poll_server
/// when the bot state is ReadyForJob.
void NetworkManager::request_command_page(const int request_page_number, CommandPage &command_page) {
    char payload[64];
    snprintf(payload, sizeof(payload), R"({"status":"Working",need_page:%i})", request_page_number);

    post_bot_poll(payload, &command_page);
}

/// Posts to /bot-poll with the specified payload. Parses into command_page, if a command page is received
/// and command_page is provided.
void NetworkManager::post_bot_poll(const char *payload, CommandPage *command_page) {
    if (WiFi.status() != WL_CONNECTED) {
        connect();
    }

    // We use HTTP instead of HTTPS because the security is irrelevant and it saves a significant amount of memory.
    if (!m_http.begin(m_client, "http://uthbees.nfshost.com/drawbot/api/bot-poll")) {
        Serial.printf("[HTTP] Unable to connect to server\n");
        return;
    }

    m_http.addHeader("Content-Type", "application/json");
    Serial.print("[HTTP] Sending POST request to /bot-poll...\n");

    const int response_code = m_http.POST(payload);

    // response_code will be negative on error, or the HTTP status code on success (e.g., 200)
    if (response_code > 0) {
        Serial.printf("[HTTP] POST code: %d\n", response_code);

        // The server only responds to /bot-poll with a 200 when it's sending a command page.
        // (Otherwise it's 204 on success.)
        if (response_code == HTTP_CODE_OK) {
            if (command_page != nullptr) {
                command_page->parse_from_response(m_http.getStream(), m_http.getSize());
            } else {
                Serial.print(
                    "[HTTP] Error: Received page from server but command_page was nullptr. Discarding page.\n");
            }
        }
    } else {
        Serial.printf("[HTTP] POST failed, error: %s\n", HTTPClient::errorToString(response_code).c_str());
    }

    m_http.end();
}
