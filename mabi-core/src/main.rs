use pwm_pca9685::Channel;
use std::sync::{Arc, Mutex};
use std::thread;

mod arm;
mod datapack;
mod gamepad;
mod gateway;
mod sensor;
mod servo;

use arm::Arm;
use servo::Servo;

fn main() -> ! {
    let arm = Arm {
        base: Servo::new(Channel::C5, (0, 180), 90),
        shoulder: Servo::new(Channel::C7, (0, 180), 145),
        elbow: Servo::new(Channel::C6, (0, 180), 165),
        wrist_vertical: Servo::new(Channel::C9, (0, 180), 0),
        wrist_horizontal: Servo::new(Channel::C8, (0, 180), 0),
        claw: Servo::new(Channel::C10, (0, 70), 0),
        speed: 1.,
    };
    let shared_arm = Arc::new(Mutex::new(arm));

    let gateway_arm = shared_arm.clone();
    thread::spawn(move || gateway::thread_loop(gateway_arm));

    let gamepad_arm = shared_arm.clone();
    thread::spawn(move || gamepad::thread_loop(gamepad_arm));

    let sensor_arm = shared_arm.clone();
    thread::spawn(move || sensor::thead_loop(sensor_arm));

    loop {}
}
