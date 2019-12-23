//#include <Arduino.h>
//
//#include <AccelStepper.h>
//#include <MultiStepper.h>
//
////#include "geometry/triangle.h"
//
//
//
//#define E0_STEP_PIN        26
//#define E0_DIR_PIN         28
//#define E0_ENABLE_PIN      24
//
//#define E1_STEP_PIN        36
//#define E1_DIR_PIN         34
//#define E1_ENABLE_PIN      30
//
//#define Z_STEP_PIN         46
//#define Z_DIR_PIN          48
//#define Z_ENABLE_PIN       62
//
//#define X_STEP_PIN         54
//#define X_DIR_PIN          55
//#define X_ENABLE_PIN       38
//
//#define Y_STEP_PIN         60
//#define Y_DIR_PIN          61
//#define Y_ENABLE_PIN       56
//
////#define E0_STEP_PIN         9 // uno
////#define E0_DIR_PIN          8 // uno
////#define E0_ENABLE_PIN       7 // uno
//
//#define LED_PIN      13
//
//
//
//int step = 1;
//int x_step = 1;
//int y_step = 1;
//long multiplayer = 16l;
//long dist = 200l * multiplayer * 2;
//float speed = 200.0f * multiplayer;
//int dir = 1;
//
//bool work_in_progress = false;
//
//
//long moveToPosition = 0;
//
//int print_outputs = 0;
//
//
//
//AccelStepper e0_stepper(AccelStepper::DRIVER, E0_STEP_PIN, E0_DIR_PIN); // use functions to step
//AccelStepper e1_stepper(AccelStepper::DRIVER, E1_STEP_PIN, E1_DIR_PIN); // use functions to step
//AccelStepper x_stepper(AccelStepper::DRIVER, X_STEP_PIN, X_DIR_PIN); // use functions to step
//AccelStepper y_stepper(AccelStepper::DRIVER, Y_STEP_PIN, Y_DIR_PIN); // use functions to step
//AccelStepper z_stepper(AccelStepper::DRIVER, Z_STEP_PIN, Z_DIR_PIN); // use functions to step
//MultiStepper multi_stepper;
//
//bool command_to_stepper(AccelStepper &stepper, int &step) {
//    bool should_sleep = false;
//    if (stepper.distanceToGo() == 0l) {
//        should_sleep = true;
//        if (step == 1) {
//            step = 2;
//            stepper.moveTo(dist);
//            stepper.setSpeed(speed);
//        } else {
//            step = 1;
//            stepper.moveTo(0l);
//            stepper.setSpeed(-speed);
//        }
//    }
//    return should_sleep;
//}
//
//void setup()
//{
//    Serial.begin(9600);
//    while(!Serial) {
//        ; //wait for Serial port to connect. Needed for Leonardo only.
//    }
//
//    multi_stepper.addStepper(x_stepper);
//    multi_stepper.addStepper(y_stepper);
//
//    pinMode(E0_ENABLE_PIN, OUTPUT);
//    digitalWrite(E0_ENABLE_PIN, LOW);
//
//    pinMode(E1_ENABLE_PIN, OUTPUT);
//    digitalWrite(E1_ENABLE_PIN, LOW);
//
//    pinMode(X_ENABLE_PIN, OUTPUT);
//    digitalWrite(X_ENABLE_PIN, LOW);
//
//    pinMode(Y_ENABLE_PIN, OUTPUT);
//    digitalWrite(Y_ENABLE_PIN, LOW);
//
//    pinMode(Z_ENABLE_PIN, OUTPUT);
//    digitalWrite(Z_ENABLE_PIN, LOW);
//
////    pinMode(4, OUTPUT);
////    pinMode(2, OUTPUT);
//
////    digitalWrite(4, LOW);
////    digitalWrite(2, HIGH);
//
//    pinMode(LED_PIN, OUTPUT);
//    digitalWrite(LED_PIN, HIGH);
//
//    e0_stepper.setMaxSpeed(3200);
////    x_stepper.setAcceleration(1);
//
//    e1_stepper.setMaxSpeed(3200);
////    y_stepper.setAcceleration(1);
//
//    x_stepper.setMaxSpeed(3200);
//
//    y_stepper.setMaxSpeed(3200);
//
//    z_stepper.setMaxSpeed(3200);
//
//}
//
//
//void loop() {
//    if(x_step == 1) {
//        e0_stepper.setSpeed(speed);
//        e1_stepper.setSpeed(speed);
//        x_stepper.setSpeed(speed);
//        y_stepper.setSpeed(speed);
//        z_stepper.setSpeed(speed);
//        x_step = 0;
//    }
//    e0_stepper.runSpeed();
//    e1_stepper.runSpeed();
//    x_stepper.runSpeed();
//    y_stepper.runSpeed();
//    z_stepper.runSpeed();
//}
//
//
////void loop()
////{
////    //todo: выяснить почему при таком коде идёт рассинхрон каждый круг на 1 шаг
//////    if(!work_in_progress) {
////    bool x_sleep = command_to_stepper(x_stepper, x_step);
////    bool y_sleep = command_to_stepper(y_stepper, y_step);
////
////
////    if (x_sleep || y_sleep) {
////        Triangle triangle;
////        String str = "sss" + String(triangle.get_ac());
////        Serial.println(str);
////
////        Serial.println("x_sleep = " + String(x_sleep) + ", y_sleep = " + String(y_sleep));
////        Serial.println("x_distance = " + String(x_stepper.distanceToGo()) + ", y_distance = " +
////                       String(y_stepper.distanceToGo()));
////        delay(1000);
////        dir = 1;
////    }
//////    }
////
////    long before_x_step = x_stepper.distanceToGo();
////    long before_y_step = y_stepper.distanceToGo();
////
////    bool x_done = x_stepper.runSpeed();
////    bool y_done = y_stepper.runSpeed();
////
////    long after_x_step = x_stepper.distanceToGo();
////    long after_y_step = y_stepper.distanceToGo();
////
//////    if(print_outputs < 200) {
//////        Serial.println("x_before = " + String(before_x_step) + ", y_before = " + String(before_y_step) + "; x_after = " + String(after_x_step) + ", y_after = " + String(after_y_step));
//////        print_outputs++;
//////    }
////
////
////
//////    work_in_progress = x_done || y_done;
////    long x_dst = x_stepper.distanceToGo();
////    long y_dst = y_stepper.distanceToGo();
////    if(y_dst != x_dst && dir == 1) {
////        dir = 2;
////        Serial.println("alarm, x = " + String(x_dst) + ", y = " + String(y_dst));
////    }
////}
