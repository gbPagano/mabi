use bincode::{deserialize, serialize};
use gilrs::{Axis, Button, EventType, Gilrs};
use pwm_pca9685::Channel;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

mod servo;
mod arm;
mod datapack;

use servo::Servo;
use arm::Arm;
use datapack::*;


fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    println!("teste");
    for (id, gamepad) in gilrs.gamepads() {
        println!("{} id={id} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut arm = Arm {
        base: Servo::new(Channel::C5, (0, 180), 90),
        shoulder: Servo::new(Channel::C7, (0, 180), 145),
        elbow: Servo::new(Channel::C6, (0, 180), 165),
        wrist_vertical: Servo::new(Channel::C9, (0, 180), 0),
        wrist_horizontal: Servo::new(Channel::C8, (0, 180), 0),
        claw: Servo::new(Channel::C10, (0, 70), 0),
        speed: 1.,
    };

    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap(); // Endereço local do sender
    let receiver_addr = "192.168.15.3:13129"; // Endereço do receiver

    let mut buf = [0; 128];

    let mut msg_counter = 1;
    loop {
        arm.step();

        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        //println!("Received {} bytes from {}", amt, src);

        if let Ok(datapack) = deserialize::<SensorDataPack>(&buf[..amt]) {
            let max_delta = 15.0; // Limite máximo de variação por ciclo
            let delta = (datapack.angles.roll - arm.claw.real_pos) / 2.0;

            // Limita o delta para evitar mudanças bruscas
            let delta = if delta.abs() > max_delta {
                delta.signum() * max_delta
            } else {
                delta
            };
            //arm.claw.real_pos += (datapack.angles.roll - arm.claw.real_pos) / 2.;
            //arm.claw.real_pos += delta;
            //arm.claw.speed = delta.signum() * 3. * arm.speed;
            arm.claw.real_pos = datapack.angles.roll;
            //arm.claw.real_pos += datapack.gyro.roll * 1000.;
            dbg!(datapack);
        }

        let datapack = DataPack {
            idx: msg_counter,
            on: [0; 16],
            off: arm.get_duty_array(),
        };
        let encoded = serialize(&datapack).expect("Failed to serialize struct");

        socket.send_to(&encoded, receiver_addr).unwrap();
        //println!("Struct sent! :: {:?}", datapack);
        msg_counter += 1;
        //thread::sleep(Duration::from_millis(15));

        arm.print_angles();
        //dbg!(&arm.speed);
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::AxisChanged(axis, val, _) => {
                    let val = if val.abs() > 0.1 { val } else { 0.0 };
                    match axis {
                        Axis::LeftStickY => {
                            println!("Left Stick Y moved: {val}");
                            arm.elbow.speed = val * 1.;
                        }
                        Axis::RightStickY => {
                            println!("Right Stick Y moved: {val}");
                            arm.shoulder.speed = -val * 1.;
                        }
                        _ => (),
                    }
                }
                EventType::ButtonChanged(button, val, _) => match button {
                    Button::RightTrigger2 => {
                        //println!("pressing: R2 :: {val}");
                        arm.base.speed = val * 2. * arm.speed;
                    }
                    Button::LeftTrigger2 => {
                        //println!("pressing: L2 :: {val}");
                        arm.base.speed = -val * 2. * arm.speed;
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
                },
                EventType::ButtonPressed(Button::DPadUp, ..) => {
                    arm.speed += 0.2;
                    arm.speed = arm.speed.clamp(0.2, 3.0);
                }
                EventType::ButtonPressed(Button::DPadDown, ..) => {
                    arm.speed -= 0.2;
                    arm.speed = arm.speed.clamp(0.2, 3.0);
                }
                _ => (),
            };
        }
    }
}


