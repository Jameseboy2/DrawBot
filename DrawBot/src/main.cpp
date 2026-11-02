#include "main.h"

#include "Pins.h"
#include "position/Point.h"
#include "position/PositionManager.h"
#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>

constexpr auto ssid = "BYUI_Visitor";
constexpr auto password = "";

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


    WiFi.begin(ssid, password);
    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
    }

    WiFiClient client;
    HTTPClient http;

    if (http.begin(client, "(https://postman-echo.com/get?foo=bar)")) {
        Serial.print("[HTTP] Sending GET request...\n");

        // Start connection and send HTTP header
        const int httpCode = http.GET();

        // httpCode will be negative on error, or the HTTP status code on success (e.g., 200)
        if (httpCode > 0) {
            Serial.printf("[HTTP] GET code: %d\n", httpCode);

            if (httpCode == HTTP_CODE_OK) {
                const String payload = http.getString();   // Get the server response body
                Serial.println("--- Server Response ---");
                Serial.println(payload);
                Serial.println("-----------------------");
            }
        } else {
            Serial.printf("[HTTP] GET failed, error: %s\n", http.errorToString(httpCode).c_str());
        }

        http.end(); // Close connection
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
