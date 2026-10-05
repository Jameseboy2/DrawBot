#include "PositionManager.h"

#include <math.h>

#include "Point.h"
#include "../Pins.h"

PositionManager::PositionManager() {
    // Assume we're in the default starting position.
    this->left_cable_steps = len_mm_to_steps(get_left_cable_len_mm(START_X_MM, START_Y_MM));
    this->right_cable_steps = len_mm_to_steps(get_right_cable_len_mm(START_X_MM, START_Y_MM));
}

void PositionManager::draw_straight_vector(const Point p1, const Point p2) {
    const float dx = p2.x - p1.x;
    const float dy = p2.y - p1.y;
    const float distance = sqrt(dx * dx + dy * dy);

    const int segment_count = max(1, static_cast<int>(ceil(distance / SEGMENT_LENGTH_MM)));

    for (int i = 0; i <= segment_count; i++) {
        const float fraction_moved = static_cast<float>(i) / segment_count;
        const float target_x = p1.x + fraction_moved * dx;
        const float target_y = p1.y + fraction_moved * dy;

        move_to_point(target_x, target_y);
    }
}

// Move to a target point by turning both motors proportionally using a variant of Bresenham's algorithm.
void PositionManager::move_to_point(const float target_x, const float target_y) {
    const long target_left_steps = get_left_cable_len_steps(target_x, target_y);
    const long target_right_steps = get_right_cable_len_steps(target_x, target_y);

    const long delta_left = target_left_steps - left_cable_steps;
    const long delta_right = target_right_steps - right_cable_steps;

    digitalWrite(Pins::DIR_1, delta_left >= 0 ? LEFT_LENGTHEN_DIR : !LEFT_LENGTHEN_DIR);
    digitalWrite(Pins::DIR_2, delta_right >= 0 ? RIGHT_LENGTHEN_DIR : !RIGHT_LENGTHEN_DIR);

    const long abs_delta_left = abs(delta_left);
    const long abs_delta_right = abs(delta_right);
    const long max_steps = max(abs_delta_left, abs_delta_right);

    long acc_left = 0;
    long acc_right = 0;

    for (long i = 0; i < max_steps; i++) {
        acc_left += abs_delta_left;
        acc_right += abs_delta_right;

        bool pulse_left = false;
        bool pulse_right = false;

        if (acc_left >= max_steps) {
            acc_left -= max_steps;
            digitalWrite(Pins::PUL_1, HIGH);
            pulse_left = true;
        }
        if (acc_right >= max_steps) {
            acc_right -= max_steps;
            digitalWrite(Pins::PUL_2, HIGH);
            pulse_right = true;
        }

        delayMicroseconds(PULSE_DELAY);

        if (pulse_left) digitalWrite(Pins::PUL_1, LOW);
        if (pulse_right) digitalWrite(Pins::PUL_2, LOW);

        delayMicroseconds(PULSE_DELAY);
    }

    left_cable_steps = target_left_steps;
    right_cable_steps = target_right_steps;
}

float PositionManager::get_left_cable_len_mm(const float x, const float y) {
    return sqrt(x * x + y * y);
}

float PositionManager::get_right_cable_len_mm(const float x, const float y) {
    const float dx = BOARD_WIDTH_MM - x;
    return sqrt(dx * dx + y * y);
}

float PositionManager::get_left_cable_len_steps(const float x, const float y) {
    return len_mm_to_steps(get_left_cable_len_mm(x, y));
}

float PositionManager::get_right_cable_len_steps(const float x, const float y) {
    return len_mm_to_steps(get_right_cable_len_mm(x, y));
}

long PositionManager::len_mm_to_steps(const double length_mm) {
    return round(length_mm * STEPS_PER_MM);
}
