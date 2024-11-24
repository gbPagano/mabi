use gilrs::{Axis, Button, Event, EventType, GamepadId, Gilrs};
use mabi_rs::servo::Servo;
use pwm_pca9685::{Address, Channel, Pca9685};
use rppal::i2c::I2c;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    println!("teste");
    for (id, gamepad) in gilrs.gamepads() {
        println!("{} id={id} is {:?}", gamepad.name(), gamepad.power_info());
    }

    // initialize pwm controller
    let dev = I2c::new().unwrap();
    let address = Address::default();
    let pwm = Pca9685::new(dev, address).unwrap();

    let mut arm = Arm {
        controller: pwm,
        base: Servo::new(Channel::C5, (0, 180), 90),
        shoulder: Servo::new(Channel::C1, (0, 180), 90),
        elbow: Servo::new(Channel::C1, (0, 180), 90),
        wrist_vertical: Servo::new(Channel::C1, (0, 180), 90),
        wrist_horizontal: Servo::new(Channel::C1, (0, 180), 90),
        claw: Servo::new(Channel::C1, (0, 180), 90),
    };

    arm.init();
    loop {
        //arm.update_positions();
        //if let Some((_, gamepad)) = gilrs.gamepads().next() {
        //    //if let Some(data) = gamepad.button_data(Button::RightTrigger2) {
        //    //println!("{:?}", data); }
        //    //let code = gamepad.button_code(Button::RightTrigger2).unwrap();
        //    let data = gamepad
        //        .state()
        //        .value(gamepad.button_code(Button::RightTrigger2).unwrap());
        //    println!("{:?}", data);
        //}
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::AxisChanged(axis, val, _) if val.abs() > 0.2 => {
                    //let val = if val.abs() > 0.2 { val } else { 0.0 };
                    match axis {
                        Axis::LeftStickX => {
                            println!("Left Stick X moved: {val}");
                        }
                        Axis::LeftStickY => {
                            println!("Left Stick Y moved: {val}");
                        }
                        Axis::RightStickX => {
                            println!("Right Stick X moved: {val}");
                        }
                        Axis::RightStickY => {
                            println!("Right Stick Y moved: {val}");
                        }
                        _ => (),
                    }
                }
                EventType::ButtonChanged(button, val, _) => match button {
                    Button::RightTrigger2 => {
                        println!("pressing: R2 :: {val}");
                         arm.base.step_speed = val;
                    }
                    Button::LeftTrigger2 => {
                        println!("pressing: L2 :: {val}");
                         arm.base.step_speed = -val;
                    }
                    _ => (),
                },
                _ => (),
            };
        }
    }
}

struct Arm {
    controller: Pca9685<I2c>,
    pub base: Servo,
    pub shoulder: Servo,
    pub elbow: Servo,
    pub wrist_vertical: Servo,
    pub wrist_horizontal: Servo,
    pub claw: Servo,
}

impl Arm {
    pub fn init(&mut self) {
        // This corresponds to a frequency of 60 Hz
        self.controller.set_prescale(121).unwrap();
        self.controller.enable().unwrap();

        self.update_positions();
    }

    pub fn step(&mut self) {
        self.base.step();
        self.shoulder.step();
        self.elbow.step();
        //self.wrist_vertical.step();
        //self.wrist_horizontal.step();
        //self.claw.step();

        self.update_positions();
    }

    fn update_positions(&mut self) {
        self.controller
            .set_channel_on_off(self.base.channel, 0, self.base.curr_duty)
            .unwrap();
        self.controller
            .set_channel_on_off(self.shoulder.channel, 0, self.shoulder.curr_duty)
            .unwrap();
        self.controller
            .set_channel_on_off(self.elbow.channel, 0, self.elbow.curr_duty)
            .unwrap();
        //self.controller
        //    .set_channel_on_off(self.wrist_vertical.channel, 0, self.wrist_vertical.curr_duty)
        //    .unwrap();
        //self.controller
        //    .set_channel_on_off(self.wrist_horizontal.channel, 0, self.wrist_horizontal.curr_duty)
        //    .unwrap();
        //self.controller
        //    .set_channel_on_off(self.claw.channel, 0, self.claw.curr_duty)
        //    .unwrap();
    }
}
