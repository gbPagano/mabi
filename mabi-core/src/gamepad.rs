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
                EventType::ButtonPressed(Button::DPadUp, ..) => {
                    handle_dpad_event(Button::DPadUp, &mut arm)
                }
                EventType::ButtonPressed(Button::DPadDown, ..) => {
                    handle_dpad_event(Button::DPadDown, &mut arm)
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
            arm.elbow.speed = val;
        }
        Axis::RightStickY => {
            arm.shoulder.speed = -val;
        }
        _ => (),
    }
}

fn handle_button_event(button: Button, val: f32, arm: &mut Arm) {
    match button {
        Button::RightTrigger2 => {
            arm.base.speed = val * 2.0 * arm.speed;
        }
        Button::LeftTrigger2 => {
            arm.base.speed = -val * 2.0 * arm.speed;
        }
        Button::South => {
            arm.wrist_vertical.speed = -val * 0.75 * arm.speed;
        }
        Button::North => {
            arm.wrist_vertical.speed = val * 0.75 * arm.speed;
        }
        Button::RightTrigger => {
            arm.wrist_horizontal.speed = -val * 0.85 * arm.speed;
        }
        Button::LeftTrigger => {
            arm.wrist_horizontal.speed = val * 0.85 * arm.speed;
        }
        Button::East => {
            arm.claw.speed = val * 0.75 * arm.speed;
        }
        Button::West => {
            arm.claw.speed = -val * 0.75 * arm.speed;
        }
        _ => (),
    }
}

fn handle_dpad_event(button: Button, arm: &mut Arm) {
    match button {
        Button::DPadUp => {
            arm.speed += 0.2;
            arm.speed = arm.speed.clamp(0.2, 3.0);
        }
        Button::DPadDown => {
            arm.speed -= 0.2;
            arm.speed = arm.speed.clamp(0.2, 3.0);
        }
        _ => (),
    }
}
