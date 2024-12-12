use gilrs::{Axis, Button, EventType, Gilrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::arm::Arm;

pub fn thread_loop(shared_arm: Arc<Mutex<Arm>>) -> ! {
    let mut gilrs = Gilrs::new().unwrap();
    for (id, gamepad) in gilrs.gamepads() {
        println!("{} id={id} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        while let Some(event) = gilrs.next_event() {
            let mut arm = shared_arm.lock().unwrap();

            match event.event {
                EventType::AxisChanged(axis, val, _) => handle_axis_event(axis, val, &mut arm),
                EventType::ButtonChanged(button, val, _) => {
                    handle_button_event(button, val, &mut arm)
                }
                _ => (),
            }
        }
        thread::sleep(Duration::from_millis(15));
    }
}

fn handle_axis_event(axis: Axis, val: f32, arm: &mut Arm) {
    let val = if val.abs() > 0.1 { val } else { 0.0 };
    match axis {
        Axis::LeftStickY => {
            arm.elbow.speed = val * 60.;
        }
        Axis::RightStickY => {
            arm.shoulder.speed = -val * 60.;
        }
        _ => (),
    }
}

fn handle_button_event(button: Button, val: f32, arm: &mut Arm) {
    match button {
        Button::RightTrigger2 => {
            arm.base.speed = val * 90.;
        }
        Button::LeftTrigger2 => {
            arm.base.speed = -val * 90.;
        }
        Button::South => {
            arm.wrist_vertical.speed = -val * 90.;
        }
        Button::North => {
            arm.wrist_vertical.speed = val * 90.;
        }
        Button::RightTrigger => {
            arm.wrist_horizontal.speed = -val * 90.;
        }
        Button::LeftTrigger => {
            arm.wrist_horizontal.speed = val * 90.;
        }
        Button::East => {
            arm.claw.speed = val * 90.;
        }
        Button::West => {
            arm.claw.speed = -val * 90.;
        }
        Button::DPadUp => {
            arm.speed_multiplier += val * 0.2;
            arm.speed_multiplier = arm.speed_multiplier.clamp(0.2, 3.0);
        }
        Button::DPadDown => {
            arm.speed_multiplier -= val * 0.2;
            arm.speed_multiplier = arm.speed_multiplier.clamp(0.2, 3.0);
        }
        _ => (),
    }
}
