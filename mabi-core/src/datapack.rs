use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct DataPack {
    pub idx: u32,
    pub on: [u16; 16],
    pub off: [u16; 16],
}

#[derive(Debug, Deserialize)]
pub struct SensorDataPack {
    pub gyro: Gyro,
    pub angles: Angles,
}

#[derive(Deserialize, Debug)]
pub struct Gyro {
    pub roll: f32,
    pub pitch: f32,
}

#[derive(Deserialize, Debug)]
pub struct Angles {
    pub roll: f32,
    pub pitch: f32,
}
