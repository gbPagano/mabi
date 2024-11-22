use embedded_hal::delay::DelayNs;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use mpu6050::*;

pub const PI: f32 = core::f32::consts::PI;
pub const PI_180: f32 = PI / 180.0;

fn main() {
    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio21; // GPIO21 padrão para SDA no ESP32
    let scl = peripherals.pins.gpio22; // GPIO22 padrão para SCL no ESP32

    let config = I2cConfig::default();
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();

    let mut delay = Ets;
    let mut mpu = Mpu6050::new(i2c);
    //let mut mpu = Mpu6050::new_with_sens(i2c, device::AccelRange::G2, device::GyroRange::D500);
    mpu.init(&mut delay).unwrap();

    println!("Starting measures, all should be zero");
    let mut measures_gyro = vec![];
    let mut measures_acc = vec![];
    for _ in 0..2000 {
        let gyro = mpu.get_gyro().unwrap();
        let acc = mpu.get_acc().unwrap();

        measures_gyro.push(gyro);
        measures_acc.push(acc);

        delay.delay_ms(1u32);
    }

    let mean_gyro: Vec<f32> = measures_gyro
        .iter()
        .fold(vec![0.0; 3], |mut acc, v| {
            for (i, &val) in v.iter().enumerate() {
                acc[i] += val;
            }
            acc
        })
        .into_iter()
        .map(|s| s / measures_gyro.len() as f32)
        .collect();
    println!("Mean gyro measures: {:?}", mean_gyro);

    let mean_acc: Vec<f32> = measures_acc
        .iter()
        .fold(vec![0.0; 3], |mut acc, v| {
            for (i, &val) in v.iter().enumerate() {
                acc[i] += val;
            }
            acc
        })
        .into_iter()
        .map(|s| s / measures_acc.len() as f32)
        .collect();
    println!("Mean acc measures: {:?}", mean_acc);

    println!(
        "const BIAS_GYRO: (f32, f32, f32) = ({}, {}, {})",
        -mean_gyro[0], -mean_gyro[1], -mean_gyro[2]
    );
    println!(
        "const BIAS_ACC: (f32, f32, f32) = ({}, {}, {})",
        -mean_acc[0], -mean_acc[1], -mean_acc[2]
    );
}
