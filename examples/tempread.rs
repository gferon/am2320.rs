use am2320::*;

fn main() -> Result<(), CommunicationError> {
    println!("{:?}", AM2320::read());
    Ok(())
}
