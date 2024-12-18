use anyhow::Result;
use bincode::deserialize;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use pwm_pca9685::{Address, Pca9685};
use serde::Deserialize;
use std::net::UdpSocket;

mod wifi;

#[derive(Deserialize, Debug)]
struct DataPack {
    idx: u32,
    on: [u16; 16],
    off: [u16; 16],
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;
    wifi::connect_wifi(&mut wifi)?;

    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    let config = I2cConfig::default();
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;

    let mut controller = Pca9685::new(i2c, Address::default()).unwrap();
    controller.set_prescale(121).unwrap(); // This corresponds to a frequency of 50 Hz
    controller.enable().unwrap();

    let socket = UdpSocket::bind("0.0.0.0:13129")?;
    let mut buf = [0; 128];

    let mut last_msg_idx = 0;
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;

        if let Ok(datapack) = deserialize::<DataPack>(&buf[..amt]) {
            println!(
                "Received :: idx={} duty_arr={:?} from={}",
                datapack.idx, datapack.off, src
            );

            if datapack.idx > last_msg_idx || datapack.idx <= 50 {
                controller
                    .set_all_on_off(&datapack.on, &datapack.off)
                    .unwrap();
                last_msg_idx = datapack.idx;
            }
        }
    }
}
