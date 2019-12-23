#include <Arduino.h>
#include <AccelStepper.h>
#include "geometry/moving.h"


#define Z_STEP_PIN         46
#define Z_DIR_PIN          48
#define Z_ENABLE_PIN       62

AccelStepper z_stepper(AccelStepper::DRIVER, Z_STEP_PIN, Z_DIR_PIN);
DriverState driver_state = DriverState();

void set_speed_and_distance_for_stepper(AccelStepper* stepper, float speed, float maxSpeed) {
    if(speed == 0 || maxSpeed == 0) {
        digitalWrite(Z_ENABLE_PIN, HIGH);
    } else {
        Serial.println("setting speed to: " + String(speed) + " and max speed to: " + String(maxSpeed));
        digitalWrite(Z_ENABLE_PIN, LOW);
        stepper->setMaxSpeed(maxSpeed);
        stepper->setSpeed(speed);
    }
}

void calc_state_move(DriverState* state, long x, long y) {
    Point p = Point(x, y);
    *state = state->move_to(p);
    Serial.println(state->length(driver::LEFT_BOT));
}

void setup()
{
    Serial.begin(9600);
    while(!Serial) {
        ; //wait for Serial port to connect. Needed for Leonardo only.
    }


    pinMode(Z_ENABLE_PIN, OUTPUT);
    digitalWrite(Z_ENABLE_PIN, LOW);

    set_speed_and_distance_for_stepper(&z_stepper, 3200, 3200);
}




void loop()
{
    if (Serial.available() != 0) {
        long x = Serial.readStringUntil(' ').toInt();
        long y = Serial.readStringUntil('\n').toInt();

        Serial.println("x = " + String(x) + "; y = " + String(y));

//        calc_state_move(&driver_state, x, y);
        set_speed_and_distance_for_stepper(&z_stepper, x, y);
    }

//if (x==0) {
//    long multiplayer = 16l;
//    float speed = 200.0f * multiplayer;
////    z_stepper.setSpeed(200 * 16l);
//    z_stepper.setMaxSpeed(3200);
//    z_stepper.setSpeed(speed);
//    x = 1;
//}
    z_stepper.runSpeed();
}