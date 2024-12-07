use bincode::serialize;
use gilrs::{Axis, Button, EventType, Gilrs};
use mabi_rs::servo::Servo;
use pwm_pca9685::Channel;
use serde::Serialize;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    println!("teste");
    for (id, gamepad) in gilrs.gamepads() {
        println!("{} id={id} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut arm = Arm {
        base: Servo::new(Channel::C5, (0, 180), 90),
        shoulder: Servo::new(Channel::C1, (0, 180), 90),
        elbow: Servo::new(Channel::C1, (0, 180), 90),
        wrist_vertical: Servo::new(Channel::C8, (0, 180), 0),
        wrist_horizontal: Servo::new(Channel::C7, (0, 180), 0),
        claw: Servo::new(Channel::C1, (0, 180), 90),
    };

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); // Endereço local do sender
    let receiver_addr = "192.168.15.2:13129"; // Endereço do receiver
    let mut msg_counter = 1;
    loop {
        let start = Instant::now();
        arm.step();

        let datapack = DataPack {
            idx: msg_counter,
            on: [0; 16],
            off: arm.get_duty_array(),
        };
        let encoded = serialize(&datapack).expect("Failed to serialize struct");

        socket.send_to(&encoded, receiver_addr).unwrap();
        //println!("Struct sent! :: {:?}", datapack);
        msg_counter += 1;
        thread::sleep(Duration::from_millis(15));
    
        arm.print_angles();
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::AxisChanged(axis, val, _) => {
                    let val = if val.abs() > 0.04 { val } else { 0.0 };
                    match axis {
                        Axis::LeftStickX => {
                            //println!("Left Stick X moved: {val}");
                        }
                        Axis::LeftStickY => {
                            println!("Left Stick Y moved: {val}");
                            arm.wrist_vertical.speed = val * 2.;
                        }
                        Axis::RightStickX => {
                            //println!("Right Stick X moved: {val}");
                        }
                        Axis::RightStickY => {
                            println!("Right Stick Y moved: {val}");
                            arm.wrist_horizontal.speed = val * 2.;
                        }
                        _ => (),
                    }
                }
                EventType::ButtonChanged(button, val, _) => match button {
                    Button::RightTrigger2 => {
                        //println!("pressing: R2 :: {val}");
                        arm.base.speed = val * 2.;
                    }
                    Button::LeftTrigger2 => {
                        //println!("pressing: L2 :: {val}");
                        arm.base.speed = -val * 2.;
                    }
                    _ => (),
                },
                _ => (),
            };
        }
        let duration = start.elapsed(); // Calcula o tempo decorrido
                                        //println!("levou {:?}", duration);
    }
}

#[derive(Serialize, Debug)]
struct DataPack {
    idx: u32,
    on: [u16; 16],
    off: [u16; 16],
}

struct Arm {
    //controller: Pca9685<I2c>,
    pub base: Servo,
    pub shoulder: Servo,
    pub elbow: Servo,
    pub wrist_vertical: Servo,
    pub wrist_horizontal: Servo,
    pub claw: Servo,
}

impl Arm {
    pub fn step(&mut self) {
        self.base.step();
        self.shoulder.step();
        self.elbow.step();
        self.wrist_vertical.step();
        self.wrist_horizontal.step();
        self.claw.step();
    }

    pub fn get_duty_array(&mut self) -> [u16; 16] {
        let mut duty_array = [0; 16];

        for s in [
            &self.base,
            &self.shoulder,
            &self.elbow,
            &self.wrist_vertical,
            &self.wrist_horizontal,
            &self.claw,
        ] {
            let idx = s.get_channel_idx();
            duty_array[idx] = s.curr_duty;
        }

        duty_array
    }

    pub fn print_angles(&self) {
        println!("Angles :: Base={:?}, Shoulder={:?}, Elbow={:?}, Wrist Vert={:?}, Wrist Horiz={:?}, Claw={:?}",
            self.base.curr_angle(), self.shoulder.curr_angle(), self.elbow.curr_angle(),
            self.wrist_vertical.curr_angle(), self.wrist_horizontal.curr_angle(), self.claw.curr_angle());
    }
}
