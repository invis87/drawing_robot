//
// Created by pronvis on 2019-12-16.
//

#include "../Configuration.h"

class Field {
    private:
        long height_size;
        long width_size;

    public:
        Field(long height, long width) {
            height_size = height;
            width_size = width;
        }

        long get_height() {
            return this->height_size;
        }

        long get_width() {
            return this->width_size;
        }
};

class Point {
    public:
        long x;
        long y;

        Point(long x, long y) {
            this->x = x;
            this->y = y;
        }
};

//#define FIELD Field(WIDTH_BETWEEN_MOTORS * POINTS_PER_MILLIMETER * 10, HEIGHT_BETWEEN_MOTORS * POINTS_PER_MILLIMETER * 10);
const Field FIELD = Field(WIDTH_BETWEEN_MOTORS * POINTS_PER_MILLIMETER * 10, HEIGHT_BETWEEN_MOTORS * POINTS_PER_MILLIMETER * 10);