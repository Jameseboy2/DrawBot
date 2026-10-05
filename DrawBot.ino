const int EN_1_PIN = A0;
const int DIR_1_PIN = A1;
const int PUL_1_PIN = A2;
const int EN_2_PIN = A3;
const int DIR_2_PIN = A4;
const int PUL_2_PIN = A5;

// Determines motor speed - lower is faster.
const int PULSE_DELAY = 500;

void setup() {
    pinMode(EN_1_PIN, OUTPUT);
    pinMode(DIR_1_PIN, OUTPUT);
    pinMode(PUL_1_PIN, OUTPUT);
    pinMode(EN_2_PIN, OUTPUT);
    pinMode(DIR_2_PIN, OUTPUT);
    pinMode(PUL_2_PIN, OUTPUT);

    digitalWrite(EN_1_PIN, LOW);
    digitalWrite(EN_2_PIN, LOW);
    // Wait for the drivers to process the enable commands.
    delay(200);
}

void loop() {
    for (int i = 0; i < 500; i++) {
        digitalWrite(PUL_1_PIN, HIGH);
        digitalWrite(PUL_2_PIN, HIGH);
        delayMicroseconds(PULSE_DELAY);
        digitalWrite(PUL_1_PIN, LOW);
        digitalWrite(PUL_2_PIN, LOW);
        delayMicroseconds(PULSE_DELAY);
    }

    delay(1000);

    // Reverse direction.
    digitalWrite(DIR_1_PIN, !digitalRead(DIR_1_PIN));
    digitalWrite(DIR_2_PIN, !digitalRead(DIR_2_PIN));
    // Wait for the drivers to process the direction commands.
    delayMicroseconds(5);
}
