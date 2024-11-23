use embedded_hal::delay::DelayNs;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::peripherals::Peripherals;
use libm::{atan2f, powf, sqrtf};
use mpu6050::*;
use std::thread;
use std::time::Duration;

pub const PI: f32 = core::f32::consts::PI;
pub const PI_180: f32 = PI / 180.0;

const BIAS_GYRO: (f32, f32, f32) = (0.083, -0.019, 0.011);
const BIAS_ACC: (f32, f32, f32) = (0.024, -0.009, -0.104);
const ALPHA: f32 = 0.95;

struct KalmanFilter {
    estimate: f32,
    estimate_error: f32,
    process_noise: f32,
    measurement_noise: f32,
    last_angular_velocity: f32,
}
impl KalmanFilter {
    fn new(estimate: f32, estimate_error: f32, process_noise: f32, measurement_noise: f32) -> Self {
        KalmanFilter {
            estimate,
            estimate_error,
            process_noise,
            measurement_noise,
            last_angular_velocity: 0.0,
        }
    }

    fn estimate(
        &mut self,
        accelerometer_angle: f32,
        angular_velocity: f32,
        delta_time: f32,
    ) -> f32 {
        self.estimate += angular_velocity * delta_time;
        self.estimate_error += self.process_noise * powf(delta_time, 2.);

        let kalman_gain = self.estimate_error / (self.estimate_error + self.measurement_noise);

        self.estimate += kalman_gain * (accelerometer_angle - self.estimate);
        self.estimate_error = (1.0 - kalman_gain) * self.estimate_error;

        self.estimate
    }
}

fn filter_complementary(curr: (f32, f32), accel_angle: (f32, f32), gyro: (f32, f32), dt: f32) -> (f32, f32) {
    let (accel_angle_x, accel_angle_y) = accel_angle;
    let (gyro_x, gyro_y) = gyro;

    // Calcular os ângulos filtrados usando o filtro complementar
    let angle_x = ALPHA * (curr.0 + gyro_x * dt) + (1.0 - ALPHA) * accel_angle_x;
    let angle_y = ALPHA * (curr.1 + gyro_y * dt) + (1.0 - ALPHA) * accel_angle_y;

    (angle_x, angle_y)
}

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

    let mut kalman_filter_roll = KalmanFilter::new(0., 0.3, 0.5, 0.1);
    let mut kalman_filter_pitch = KalmanFilter::new(0., 0.3, 0.5, 0.1);

    let mut last_time = std::time::Instant::now();
    let mut last_print = std::time::Instant::now();
    let mut curr = (0., 0.);
    loop {
        let current_time = std::time::Instant::now();
        let delta_time = (current_time - last_time).as_secs_f32();
        last_time = current_time;

        let raw_gyro = mpu.get_gyro().unwrap();
        let raw_acc = mpu.get_acc().unwrap();

        let gyro = (
            raw_gyro.x + BIAS_GYRO.0,
            raw_gyro.y + BIAS_GYRO.1,
            raw_gyro.z + BIAS_GYRO.2,
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

        let angle_roll = kalman_filter_roll.estimate(angles.0, gyro.0, delta_time);
        let angle_pitch = kalman_filter_pitch.estimate(angles.1, gyro.1, delta_time);

        curr = filter_complementary(curr, angles, (gyro.0, gyro.1), delta_time);

        if (current_time - last_print).as_millis() >= 300 {
            last_print = current_time;
            println!("last dt: {:?}", delta_time);
            println!("no kalman r/p: {:?}", angles);
            println!("with kalman r/p: {:?}", (angle_roll, angle_pitch));
            println!("complementary: {:?}", curr);
        }

        thread::sleep(Duration::from_millis(5));

    }
}



        

           
           

            
