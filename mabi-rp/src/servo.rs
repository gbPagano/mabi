use pwm_pca9685::Channel;


pub struct Servo {
    pub channel: Channel,
    pub angle_range: (u8, u8),
    pub curr_duty: u16,
    pub speed: f32, // step speed
    real_pos: f32, // in angle
}

impl Servo {
    const MIN_DUTY: u16 = 103;
    const MAX_DUTY: u16 = 492;

    pub fn new(channel: Channel, angle_range: (u8, u8), start_angle: u8) -> Self {
        Servo {
            channel,
            angle_range,
            curr_duty: Self::angle_to_duty(start_angle),
            real_pos: start_angle as f32,
            speed: 0.,
        }
    }

    fn get_curr_angle(&self) -> u8 {
        (180.0
            * ((self.curr_duty - Self::MIN_DUTY) as f32 / (Self::MAX_DUTY - Self::MIN_DUTY) as f32))
            .round() as u8
    }

    fn angle_to_duty(angle: u8) -> u16 {
        ((angle as f32 / 180.0) * (Self::MAX_DUTY - Self::MIN_DUTY) as f32) as u16 + Self::MIN_DUTY
    }

    pub fn step(&mut self) {
        self.real_pos += self.speed;
        self.real_pos = self.real_pos.min(self.angle_range.1 as f32);
        self.real_pos = self.real_pos.max(self.angle_range.0 as f32);

        self.curr_duty = Servo::angle_to_duty(self.real_pos as u8);
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
            angle_range: (0, 0),
            curr_duty: Servo::MIN_DUTY,
            real_pos: 0.,
            speed: 0.,
        };
        assert_eq!(servo.get_curr_angle(), 0);

        servo.curr_duty = Servo::MAX_DUTY;
        assert_eq!(servo.get_curr_angle(), 180);

        servo.curr_duty = (Servo::MAX_DUTY - Servo::MIN_DUTY) / 2 + Servo::MIN_DUTY;
        assert_eq!(servo.get_curr_angle(), 90);
    }
}
