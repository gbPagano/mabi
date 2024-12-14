use anyhow::{bail, Result};
use embedded_hal::delay::DelayNs;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use libm::{atan2f, powf, sqrtf};
use log::info;
use mpu6050::*;
use std::io;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use bincode::serialize;
use serde::Serialize;

use core::convert::TryInto;

use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};

use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

const SSID: &str = "S22-gbp";
const PASSWORD: &str = "roteadorS22!";

pub const PI: f32 = core::f32::consts::PI;
pub const PI_180: f32 = PI / 180.0;

const BIAS_GYRO: (f32, f32, f32) = (0.083, -0.019, 0.011);
const BIAS_ACC: (f32, f32, f32) = (0.024, -0.009, -0.104);
const ALPHA: f32 = 0.95;

#[derive(Serialize)]
struct SensorDataPack {
    gyro: Gyro,
    angles: Angles,
}


#[derive(Serialize)]
struct Gyro {
    roll: f32,
    pitch: f32,

}


#[derive(Serialize)]
struct Angles {
    roll: f32,
    pitch: f32,
}




fn filter_complementary(
    curr: (f32, f32),
    accel_angle: (f32, f32),
    gyro: (f32, f32),
    dt: f32,
) -> (f32, f32) {
    let (accel_angle_x, accel_angle_y) = accel_angle;
    let (gyro_x, gyro_y) = gyro;

    // Calcular os ângulos filtrados usando o filtro complementar
    let angle_x = ALPHA * (curr.0 + gyro_x * dt) + (1.0 - ALPHA) * accel_angle_x;
    let angle_y = ALPHA * (curr.1 + gyro_y * dt) + (1.0 - ALPHA) * accel_angle_y;

    (angle_x, angle_y)
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let sda = peripherals.pins.gpio21; // GPIO21 padrão para SDA no ESP32
    let scl = peripherals.pins.gpio22; // GPIO22 padrão para SCL no ESP32

    let config = I2cConfig::default();
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();

    let mut delay = Ets;
    let mut mpu = Mpu6050::new(i2c);
    //let mut mpu = Mpu6050::new_with_sens(i2c, device::AccelRange::G2, device::GyroRange::D500);
    mpu.init(&mut delay).unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    println!("Wifi DHCP info: {:?}", ip_info);

    let server_address = "192.168.224.187:13129";
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let mut last_time = std::time::Instant::now();
    let mut last_print = std::time::Instant::now();
    let mut curr = (0., 0.);
    let (mut x, mut y) = (0., 0.);
    loop {
        let current_time = std::time::Instant::now();
        let delta_time = (current_time - last_time).as_secs_f32();
        last_time = current_time;

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

        let angles = (
            atan2f(acc.1, sqrtf(powf(acc.0, 2.) + powf(acc.2, 2.))) * 180. / PI,
            atan2f(-acc.0, sqrtf(powf(acc.1, 2.) + powf(acc.2, 2.))) * 180. / PI,
        );


        curr = filter_complementary(curr, angles, (gyro.0, gyro.1), delta_time);
        x += gyro.0 * delta_time;
        y += gyro.1 * delta_time;


        if (current_time - last_print).as_millis() >= 300 {
            last_print = current_time;
            //println!("last dt: {:?}", delta_time);
            println!("complementary: {:?}", curr);
            println!("{:#?} {:#?}", x, y);

            let angles = Angles {
                roll: curr.0,
                pitch: curr.1,
            };
            let gyro = Gyro {
                roll: gyro.0 * delta_time,
                pitch: gyro.1 * delta_time,
            };
            let datapack = SensorDataPack {gyro, angles};
            let encoded = serialize(&datapack).expect("Failed to serialize struct");
            socket.send_to(&encoded, server_address)?;
        }

        thread::sleep(Duration::from_millis(15));
    }
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}
