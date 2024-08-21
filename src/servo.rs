use pwm_pca9685::{Address, Channel, Pca9685};
use rppal::i2c::I2c;
use std::thread;
use std::time::Duration;

struct Servo {
    channel: Channel,
    speed: u8,
    angle_range: (u8, u8),
    curr_duty: u16,
}

impl Servo {
    const MIN_DUTY: u16 = 103;
    const MAX_DUTY: u16 = 492;

    pub fn new(channel: Channel, speed: u8, angle_range: (u8, u8), start_angle: u8) -> Self {
        Servo {
            channel,
            speed,
            angle_range,
            curr_duty: Self::angle_to_duty(start_angle),
        }
    }

    fn get_curr_angle(&self) -> u8 {
        (180.0
            * ((self.curr_duty - Self::MIN_DUTY) as f64 / (Self::MAX_DUTY - Self::MIN_DUTY) as f64))
            .round() as u8
    }

    fn angle_to_duty(angle: u8) -> u16 {
        ((angle as f64 / 180.0) * (Self::MAX_DUTY - Self::MIN_DUTY) as f64) as u16 + Self::MIN_DUTY
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_to_duty() {
        assert_eq!(Servo::angle_to_duty(0), Servo::MIN_DUTY);
        assert_eq!(Servo::angle_to_duty(180), Servo::MAX_DUTY);
        assert_eq!(
            Servo::angle_to_duty(90),
            (Servo::MAX_DUTY - Servo::MIN_DUTY) / 2 + Servo::MIN_DUTY
        );
    }

    #[test]
    fn test_get_curr_angle() {
        let mut servo = Servo {
            channel: Channel::C0,
            speed: 0,
            angle_range: (0, 0),
            curr_duty: Servo::MIN_DUTY,
        };
        assert_eq!(servo.get_curr_angle(), 0);

        servo.curr_duty = Servo::MAX_DUTY;
        assert_eq!(servo.get_curr_angle(), 180);

        servo.curr_duty = (Servo::MAX_DUTY - Servo::MIN_DUTY) / 2 + Servo::MIN_DUTY;
        assert_eq!(servo.get_curr_angle(), 90);
    }
}

fn main() {
    let dev = I2c::new().unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(121).unwrap();

    // It is necessary to enable the dvice.
    pwm.enable().unwrap();

    // Turn on channel 0 at 0.
    //pwm.set_channel_on(Channel::C1, 150).unwrap();
    let min = 103;
    let max = 492;

    let min_b = 103;
    let max_b = 492;

    pwm.set_channel_on_off(Channel::C1, 0, min).unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, min_b).unwrap();
    thread::sleep(Duration::from_millis(1500));
    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
    //pwm.set_channel_off(Channel::C1, 600).unwrap();
    pwm.set_channel_on_off(Channel::C1, 0, ((max - min) / 2) + min)
        .unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, ((max_b - min_b) / 2) + min_b)
        .unwrap();
    //pwm.set_channel_on_off(Channel::C5, 0, 308).unwrap();
    thread::sleep(Duration::from_millis(1500));

    pwm.set_channel_on_off(Channel::C1, 0, max).unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, max_b).unwrap();

    thread::sleep(Duration::from_millis(1000));
    pwm.set_channel_on_off(Channel::C1, 0, ((max - min) / 2) + min)
        .unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, ((max_b - min_b) / 2) + min_b)
        .unwrap();
    thread::sleep(Duration::from_millis(1000));
    let _dev = pwm.destroy(); // Get the I2C device back
}
