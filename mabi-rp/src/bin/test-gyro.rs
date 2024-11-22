use std::f64::consts::PI;
use gy521_rppal::Gy521;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the GY-521 on I2C bus 1 with the default address 0x68
    let gy521 = Gy521::new(1, 0x68)?;
    
    // Wake up the sensor
    gy521.wakeup()?;
    
    // Read raw accelerometer data
     loop {
            let (mut raw_accel, mut raw_gyro) = gy521.read_raw()?;
    println!("Raw GyroScope Data: {:?}", raw_gyro);
    
    // Normalize to g's
    raw_gyro.normalize_to_gs();
    println!("Normalized: {:?}", raw_gyro);
    
    // Calculate roll and pitch
    let ((roll_accel, pitch_accel), (roll_gyro, pitch_gyro)) = gy521.read_raw_poll_pitch()?;
    println!("Roll (Gyro): {:.2}°, Pitch (Gyro): {:.2}°", roll_gyro, pitch_gyro);

    thread::sleep(Duration::from_millis(150));
     } 
    Ok(())
}
