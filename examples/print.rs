use am2320::{Am2320, Error};
use rppal::{hal::Delay, i2c::I2c};

fn main() -> Result<(), Error> {
    let device = I2c::new().expect("could not initialize I2c on your RPi");
    let delay = Delay::new();

    let mut am2320 = Am2320::new(device, delay);
    println!("{:?}", am2320.read());
    Ok(())
}
