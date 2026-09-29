const int EN_PIN = A0;
const int DIR_PIN = A1;
const int PUL_PIN = A2;

// Determines motor speed - lower is faster.
const int PULSE_DELAY = 500;

void setup() {
    pinMode(EN_PIN, OUTPUT);
    pinMode(DIR_PIN, OUTPUT);
    pinMode(PUL_PIN, OUTPUT);

    digitalWrite(EN_PIN, LOW);
    // Wait for the driver to process the enable command.
    delay(200);
}

void loop() {
    for (int i = 0; i < 500; i++) {
        digitalWrite(PUL_PIN, HIGH);
        delayMicroseconds(PULSE_DELAY);
        digitalWrite(PUL_PIN, LOW);
        delayMicroseconds(PULSE_DELAY);
    }

    delay(1000);

    // Reverse direction
    setDirection(!digitalRead(DIR_PIN));
}

void setDirection(uint8_t dir) {
    digitalWrite(DIR_PIN, dir);
    // Wait for the driver to process the direction command.
    delayMicroseconds(5);
}
