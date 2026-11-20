#include "NetworkManager.h"

#include <ESP8266HTTPClient.h>
#include <ESP8266WiFi.h>

#include "../secrets.h"

NetworkManager::NetworkManager() {
    m_http.setReuse(true);
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

void NetworkManager::post_bot_poll(const BotState bot_state, String &response) {
    if (WiFi.status() != WL_CONNECTED) {
        connect();
    }

    // We use HTTP instead of HTTPS because the security is irrelevant and it saves a significant amount of memory.
    if (m_http.begin(m_client, "http://uthbees.nfshost.com/drawbot/api/bot-poll")) {
        m_http.addHeader("Content-Type", "application/json");

        Serial.print("[HTTP] Sending POST request to /bot-poll...\n");

        const char *bot_state_str;
        switch (bot_state) {
            case AWAITING_READY_BUTTON:
                bot_state_str = "AwaitingReset";
                break;
            case READY_FOR_JOB:
                bot_state_str = "Available";
                break;
            case WORKING:
                bot_state_str = "Working";
                break;
            default:
                bot_state_str = "Error";
                break;
        }

        char jsonPayload[32];
        snprintf(jsonPayload, sizeof(jsonPayload), "{\"status\":\"%s\"}", bot_state_str);

        const int httpCode = m_http.POST(jsonPayload);

        // httpCode will be negative on error, or the HTTP status code on success (e.g., 200)
        if (httpCode > 0) {
            Serial.printf("[HTTP] POST code: %d\n", httpCode);

            if (httpCode == HTTP_CODE_OK) {
                response = m_http.getString();
                Serial.println("--- Server Response ---");
                Serial.println(response);
                Serial.println("-----------------------");
            }
        } else {
            Serial.printf("[HTTP] POST failed, error: %s\n", m_http.errorToString(httpCode).c_str());
        }

        m_http.end();
    } else {
        Serial.printf("[HTTP] Unable to connect to server\n");
    }
}
