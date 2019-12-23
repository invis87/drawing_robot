//
// Created by pronvis on 2019-12-22.
//

#include "field.h"

enum driver {LEFT_TOP, LEFT_BOT, RIGHT_TOP, RIGHT_BOT};

class RightTriangle {
public:
    static double hypotenuse(double leg1, double leg2) {
        return sqrt(leg1 * leg1 + leg2 * leg2);
    }
};

class DriverState {
private:
    double left_top_length;
    double left_bot_length;
    double right_top_length;
//    double right_bot_length;

public:
    double right_bot_length;

    DriverState (double left_top, double left_bot, double right_top, double right_bot) {
        left_top_length =  left_top;
        left_bot_length = left_bot;
        right_top_length = right_top;
        right_bot_length = right_bot;
    }

    DriverState () {
        left_top_length =  0;
        left_bot_length = 0;
        right_top_length = 0;
        right_bot_length = 0;
    }

    double length(driver driver) {
        switch (driver) {
            case LEFT_TOP:
                return left_top_length;
            case LEFT_BOT:
                return left_bot_length;
            case RIGHT_TOP:
                return right_top_length;
            case RIGHT_BOT:
                return right_bot_length;
            default:
                return 0.0;
        }
    }

    DriverState move_to(Point p) {

        double left_top = RightTriangle::hypotenuse(p.x, p.y);

        long rest_y = FIELD.get_height() - p.y;
        double left_bot = RightTriangle::hypotenuse(p.x, rest_y);

        long rest_x = FIELD.get_width() - p.x;
        double right_top = RightTriangle::hypotenuse(rest_x, p.y);

        double right_bot = RightTriangle::hypotenuse(rest_x, rest_y);

        return {left_top, left_bot, right_top, right_bot};
    }

};