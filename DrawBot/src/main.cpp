#include "main.h"
#include "secrets.h"

#include "Pins.h"
#include "position/Point.h"
#include "position/PositionManager.h"
#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <WiFiClientSecure.h>

void main_setup() {
    // pinMode(Pins::EN_1, OUTPUT);
    // pinMode(Pins::DIR_1, OUTPUT);
    // pinMode(Pins::PUL_1, OUTPUT);
    // pinMode(Pins::EN_2, OUTPUT);
    // pinMode(Pins::DIR_2, OUTPUT);
    // pinMode(Pins::PUL_2, OUTPUT);
    //
    // digitalWrite(Pins::EN_1, LOW);
    // digitalWrite(Pins::EN_2, LOW);
    // // Wait for the drivers to process the enable commands.
    // delay(200);

    Serial.begin(115200);
    Serial.println("Initializing WiFi...");

    // The credentials come from secrets.h, which is not committed to git.
    WiFi.begin(ssid, password);
    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
    }

    WiFiClientSecure client;
    HTTPClient http;

    // Bypass SSL certificate verification (the ESP8266 has no default cert store).
    client.setInsecure();

    if (http.begin(client, "https://uthbees.nfshost.com/drawbot/api/bot-poll")) {
        http.addHeader("Content-Type", "application/json");

        Serial.print("[HTTP] Sending POST request...\n");

        const String jsonPayload = "{\"status\":\"Available\"}";
        const int httpCode = http.POST(jsonPayload);

        // httpCode will be negative on error, or the HTTP status code on success (e.g., 200)
        if (httpCode > 0) {
            Serial.printf("[HTTP] POST code: %d\n", httpCode);

            if (httpCode == HTTP_CODE_OK) {
                const String payload = http.getString();
                Serial.println("--- Server Response ---");
                Serial.println(payload);
                Serial.println("-----------------------");
            }
        } else {
            Serial.printf("[HTTP] POST failed, error: %s\n", http.errorToString(httpCode).c_str());
        }

        http.end();
    } else {
        Serial.printf("[HTTP] Unable to connect to server\n");
    }

    // auto pos_manager = PositionManager();
    // pos_manager.draw_straight_vector(Point(200.0, 400.0), Point(600.0, 400.0));
}

void main_loop() {
    // Do nothing until the reset button is pushed.
    delay(1000);
}
