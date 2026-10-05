#include "main.h"

#include "Pins.h"
#include "position/Point.h"
#include "position/PositionManager.h"

void main_setup() {
    pinMode(Pins::EN_1, OUTPUT);
    pinMode(Pins::DIR_1, OUTPUT);
    pinMode(Pins::PUL_1, OUTPUT);
    pinMode(Pins::EN_2, OUTPUT);
    pinMode(Pins::DIR_2, OUTPUT);
    pinMode(Pins::PUL_2, OUTPUT);

    digitalWrite(Pins::EN_1, LOW);
    digitalWrite(Pins::EN_2, LOW);
    // Wait for the drivers to process the enable commands.
    delay(200);

    auto pos_manager = PositionManager();
    pos_manager.draw_straight_vector(Point(200.0, 400.0), Point(600.0, 400.0));
}

void main_loop() {
    // Do nothing until the reset button is pushed.
    delay(1000);
}
