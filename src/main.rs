use gilrs::{Button, Event, EventType, Gilrs};
use mabi_rs::servo::Servo;
use pwm_pca9685::{Address, Channel, Pca9685};
use rppal::i2c::I2c;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    println!("teste");
    for (id, gamepad) in gilrs.gamepads() {
        println!("{} id={id} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let dev = I2c::new().unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    let mut arm = Arm {
        controller: pwm,
        base: Servo::new(Channel::C5, (0, 180), 90),
        shoulder: Servo::new(Channel::C5, (0, 180), 90),
        elbow: Servo::new(Channel::C5, (0, 180), 90),
        wrist_vertical: Servo::new(Channel::C5, (0, 180), 90),
        wrist_horizontal: Servo::new(Channel::C5, (0, 180), 90),
        claw: Servo::new(Channel::C5, (0, 180), 90),
    };

    arm.init();

    loop {
        arm.update_positions();
        while let Some(event) = gilrs.next_event() {
            match event {
                // Event { event: EventType::ButtonPressed(Button::South, _), .. } => {
                //     println!("pressed: X")
                // }
                Event {
                    event: EventType::AxisChanged(axis, val, _),
                    ..
                } if val.abs() > 0.2 => {
                    println!("axis {:?} : {:?}", axis, val)
                }
                Event {
                    event: EventType::ButtonChanged(Button::RightTrigger2, val, _),
                    ..
                } => {
                    println!("pressing: R2 :: {val}");
                    arm.base.step(val as f64);
                }
                Event {
                    event: EventType::ButtonChanged(Button::LeftTrigger2, val, _),
                    ..
                } => {
                    println!("pressing: L2 :: {val}");
                    arm.base.step(-val as f64);
                }
                Event {
                    event: EventType::ButtonPressed(btn, _),
                    ..
                } => {
                    println!("pressed: {:?}", btn);
                }
                Event {
                    id,
                    event: EventType::Connected,
                    ..
                } => {
                    println!("Gamepad connected :: id={id}")
                }
                Event {
                    id,
                    event: EventType::Disconnected,
                    ..
                } => {
                    println!("Gamepad disconnected :: id={id}")
                }
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

    pub fn update_positions(&mut self) {
        self.controller
            .set_channel_on_off(self.base.channel, 0, self.base.curr_duty)
            .unwrap();
    }
}
