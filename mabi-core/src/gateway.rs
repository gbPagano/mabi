use bincode::serialize;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::arm::Arm;
use crate::datapack::DataPack;

pub fn thread_loop(shared_arm: Arc<Mutex<Arm>>) -> ! {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let receiver_addr = "192.168.15.3:13129";

    let mut msg_counter = 1;
    loop {
        let mut datapack = DataPack {
            idx: msg_counter,
            on: [0; 16],
            off: [0; 16],
        };
        {
            let mut arm = shared_arm.lock().unwrap();
            arm.step();
            arm.print_angles();

            datapack.off = arm.get_duty_array();
        }
        let encoded = serialize(&datapack).expect("Failed to serialize struct");
        socket.send_to(&encoded, receiver_addr).unwrap();
        msg_counter += 1;

        thread::sleep(Duration::from_millis(15));
    }
}
