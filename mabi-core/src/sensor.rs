use bincode::deserialize;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use crate::arm::Arm;
use crate::datapack::SensorDataPack;

pub fn thead_loop(shared_arm: Arc<Mutex<Arm>>) -> ! {
    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
    let mut buf = [0; 128];

    loop {
        let (amt, _from) = socket.recv_from(&mut buf).unwrap();
        if let Ok(datapack) = deserialize::<SensorDataPack>(&buf[..amt]) {
            let mut arm = shared_arm.lock().unwrap();
            let delta = datapack.angles.roll - arm.claw.angle;
            //arm.claw.angle = datapack.angles.roll;
            arm.claw.speed = delta * 2.;
        }
    }
}
