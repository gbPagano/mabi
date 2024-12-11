use crate::servo::Servo;

pub struct Arm {
    pub base: Servo,
    pub shoulder: Servo,
    pub elbow: Servo,
    pub wrist_vertical: Servo,
    pub wrist_horizontal: Servo,
    pub claw: Servo,
    pub speed: f32,
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

    pub fn get_duty_array(&self) -> [u16; 16] {
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
