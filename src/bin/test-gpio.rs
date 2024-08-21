use rppal::i2c::I2c;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::thread;
use std::time::Duration;

fn main() {
    let dev = I2c::new().unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();

    // This corresponds to a frequency of 60 Hz.
    pwm.set_prescale(121).unwrap();

    // It is necessary to enable the device.
    pwm.enable().unwrap();

    // Turn on channel 0 at 0.
    //pwm.set_channel_on(Channel::C1, 150).unwrap();
    let min = 103;
    let max = 492;

    let min_b = 103;
    let max_b = 492;

    pwm.set_channel_on_off(Channel::C1, 0, min).unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, min_b).unwrap();
    thread::sleep(Duration::from_millis(1500));
    // Turn off channel 0 at 2047, which is 50% in
    // the range `[0..4095]`.
    //pwm.set_channel_off(Channel::C1, 600).unwrap();
    pwm.set_channel_on_off(Channel::C1, 0, ((max - min) / 2) + min).unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, ((max_b - min_b) / 2) + min_b).unwrap();
    //pwm.set_channel_on_off(Channel::C5, 0, 308).unwrap();
    thread::sleep(Duration::from_millis(1500));

    pwm.set_channel_on_off(Channel::C1, 0, max).unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, max_b).unwrap();

    thread::sleep(Duration::from_millis(1000));
    pwm.set_channel_on_off(Channel::C1, 0, ((max - min) / 2) + min).unwrap();
    pwm.set_channel_on_off(Channel::C5, 0, ((max_b - min_b) / 2) + min_b).unwrap();
    thread::sleep(Duration::from_millis(1000));
    let _dev = pwm.destroy(); // Get the I2C device back
}
