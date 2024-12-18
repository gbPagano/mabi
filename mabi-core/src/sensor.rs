use bincode::deserialize;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use crate::arm::Arm;
use crate::datapack::Angles;

pub fn thead_loop(shared_arm: Arc<Mutex<Arm>>) -> ! {
    let socket = UdpSocket::bind("0.0.0.0:13129").unwrap();
    let mut buf = [0; 128];

    loop {
        let (amt, _from) = socket.recv_from(&mut buf).unwrap();
        if let Ok(angles) = deserialize::<Angles>(&buf[..amt]) {
            let mut arm = shared_arm.lock().unwrap();
            let delta =  (30. - angles.roll) - arm.claw.angle;
            arm.claw.speed = delta * 2.;
        }
    }
}
