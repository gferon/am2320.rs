use am2320::*;
use rppal::{hal::Delay, i2c::I2c};

fn main() -> Result<(), Error> {
    // get device impl. from rppal
    let device = I2c::new()
        .expect("could not initialize I2c on your RPi, is the interface enabled in raspi-config?");
    let delay = Delay::new();

    // initialize driver
    let mut am2320 = AM2320::new(device, delay);

    // get readings
    println!("{:?}", am2320.read());
    Ok(())
}
