use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct DataPack {
    pub idx: u32,
    pub on: [u16; 16],
    pub off: [u16; 16],
}

#[derive(Deserialize, Debug)]
pub struct Angles {
    pub roll: f32,
    pub pitch: f32,
}
