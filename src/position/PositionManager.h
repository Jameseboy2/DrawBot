#pragma once
#include <Arduino.h>

#include "Point.h"

class PositionManager {
public:
    PositionManager();

    void draw_straight_vector(Point p1, Point p2);

private:
    // The number of motor steps the cables are away from being fully wound.
    long left_cable_steps;
    long right_cable_steps;


    void move_to_point(float target_x, float target_y);


    static float get_left_cable_len_mm(float x, float y);

    static float get_right_cable_len_mm(float x, float y);

    static float get_left_cable_len_steps(float x, float y);

    static float get_right_cable_len_steps(float x, float y);

    [[nodiscard]] static long len_mm_to_steps(double length_mm);


    static constexpr float BOARD_WIDTH_MM = 1500.0; // TODO: Find correct value
    static constexpr float SPOOL_DIAMETER_MM = 20.0; // TODO: Find correct value
    static constexpr float STEPS_PER_REVOLUTION = 400.0; // TODO: Find correct value
    // TODO: Find good value, or maybe get the robot to figure out its position automatically?
    static constexpr float START_X_MM = 300.0;
    // TODO: Find good value, or maybe get the robot to figure out its position automatically?
    static constexpr float START_Y_MM = 600.0;

    static constexpr bool LEFT_LENGTHEN_DIR = HIGH;
    static constexpr bool RIGHT_LENGTHEN_DIR = HIGH;

    static constexpr float SPOOL_CIRCUMFERENCE_MM = M_PI * SPOOL_DIAMETER_MM;
    static constexpr float STEPS_PER_MM = STEPS_PER_REVOLUTION / SPOOL_CIRCUMFERENCE_MM;

    // Length of a single segment of a drawn line.
    static constexpr float SEGMENT_LENGTH_MM = 1.0;
    // Motor speed - microseconds per pulse half-cycle
    static constexpr int PULSE_DELAY = 400;
};
