#pragma once

// A point on the whiteboard specified in millimeters, with (0, 0) being the top left corner.
class Point {
public:
    Point(const float x, const float y) : x(x), y(y) {
    }

    float x;
    float y;
};
