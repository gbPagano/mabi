use anyhow::Result;
use bincode::serialize;
use core::f32::consts::PI;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use libm::{atan2f, powf, sqrtf};
use mpu6050::Mpu6050;
use serde::Serialize;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};

mod wifi;

const BIAS_GYRO: (f32, f32, f32) = (0.083, -0.019, 0.011);
const BIAS_ACC: (f32, f32, f32) = (0.024, -0.009, -0.104);
const ALPHA: f32 = 0.95;
const RECEIVER_IP: Option<&str> = option_env!("CORE_IP");

#[derive(Serialize, Debug)]
struct Angles {
    roll: f32,
    pitch: f32,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::default();
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();

    let mut delay = Ets;
    let mut mpu = Mpu6050::new(i2c);
    mpu.init(&mut delay).unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;
    wifi::connect_wifi(&mut wifi)?;

    let server_address = RECEIVER_IP.expect("Environment variable not set");
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let mut last_time = Instant::now();
    let mut last_print = Instant::now();
    let mut angles = Angles {
        roll: 0.,
        pitch: 0.,
    };
    loop {
        let delta_time = last_time.elapsed().as_secs_f32();
        last_time = Instant::now();

        let raw_gyro = mpu.get_gyro().unwrap();
        let raw_acc = mpu.get_acc().unwrap();

        let gyro = (
            (raw_gyro.x + BIAS_GYRO.0) * 180. / PI,
            (raw_gyro.y + BIAS_GYRO.1) * 180. / PI,
            (raw_gyro.z + BIAS_GYRO.2) * 180. / PI,
        );
        let acc = (
            raw_acc.x + BIAS_ACC.0,
            raw_acc.y + BIAS_ACC.1,
            raw_acc.z + BIAS_ACC.2,
        );
        let acc_angles = Angles {
            roll: atan2f(acc.1, sqrtf(powf(acc.0, 2.) + powf(acc.2, 2.))) * 180. / PI,
            pitch: atan2f(-acc.0, sqrtf(powf(acc.1, 2.) + powf(acc.2, 2.))) * 180. / PI,
        };
        filter_complementary(&mut angles, &acc_angles, (gyro.0, gyro.1), delta_time);

        if last_print.elapsed().as_millis() >= 300 {
            last_print = Instant::now();
            println!("{:?}", angles);
        }
        let encoded = serialize(&angles).expect("Failed to serialize struct");
        socket.send_to(&encoded, server_address)?;

        thread::sleep(Duration::from_millis(15));
    }
}

fn filter_complementary(curr: &mut Angles, acc_angles: &Angles, gyro: (f32, f32), dt: f32) {
    let (gyro_x, gyro_y) = gyro;

    curr.roll = ALPHA * (curr.roll + gyro_x * dt) + (1.0 - ALPHA) * acc_angles.roll;
    curr.pitch = ALPHA * (curr.pitch + gyro_y * dt) + (1.0 - ALPHA) * acc_angles.pitch;
}
