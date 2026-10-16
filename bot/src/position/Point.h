#pragma once

// A point on the whiteboard specified in millimeters, with (0, 0) being the top left corner.
class Point {
public:
    Point(const int x, const int y) : x(x), y(y) {
    }

    int x;
    int y;
};
