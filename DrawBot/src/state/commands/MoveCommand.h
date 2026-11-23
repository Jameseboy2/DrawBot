#pragma once

struct MoveCommand {
    // M: Move to the specified point without drawing.
    // L: Draw a straight line to the specified point.
    char type;

    // x, y values in image pixels
    uint16_t x;
    uint16_t y;
};
