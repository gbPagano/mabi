use crate::servo::Servo;

pub struct Arm {
    pub base: Servo,
    pub shoulder: Servo,
    pub elbow: Servo,
    pub wrist_vertical: Servo,
    pub wrist_horizontal: Servo,
    pub claw: Servo,
    pub speed_multiplier: f32,
}

impl Arm {
    pub fn step(&mut self, val: f32) {
        let speed_multiplier = self.speed_multiplier;
        for servo in self.servos() {
            servo.step(val * speed_multiplier);
        }
    }

    pub fn get_duty_array(&mut self) -> [u16; 16] {
        let mut duty_array = [0; 16];

        for servo in self.servos() {
            let idx = servo.get_channel_idx();
            duty_array[idx] = servo.curr_duty;
        }

        duty_array
    }

    pub fn debug(&self) -> String {
        format!("Speed:: {:.2}, Angles :: Base={}, Shoulder={}, Elbow={}, Wrist Vert={}, Wrist Horiz={}, Claw={}",
            self.speed_multiplier, self.base.real_angle(), self.shoulder.real_angle(), self.elbow.real_angle(),
            self.wrist_vertical.real_angle(), self.wrist_horizontal.real_angle(), self.claw.real_angle())
    }

    fn servos(&mut self) -> impl Iterator<Item = &mut Servo> {
        [
            &mut self.base,
            &mut self.shoulder,
            &mut self.elbow,
            &mut self.wrist_vertical,
            &mut self.wrist_horizontal,
            &mut self.claw,
        ]
        .into_iter()
    }
}
