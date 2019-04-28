use embedded_hal::blocking::{delay, i2c};

const DEVICE_I2C_ADDR: u8 = 0x5c;

#[derive(Debug)]
pub enum Error {
    ReadError,
    WriteError,
    SensorError,
}

#[derive(Debug)]
pub struct Measurement {
    pub temperature: f64,
    pub humidity: f64,
}

pub struct AM2320<I2C, Delay> {
    device: I2C,
    delay: Delay,
}

impl<I2C, Delay, E> AM2320<I2C, Delay>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E>,
    Delay: delay::DelayUs<u16>,
{
    pub fn new(device: I2C, delay: Delay) -> Self {
        Self { device, delay }
    }

    fn crc16(data: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;
        for e in data.iter() {
            crc ^= *e as u16;
            for _i in 0..8 {
                if (crc & 0x0001) == 0x0001 {
                    crc >>= 1;
                    crc ^= 0xA001;
                } else {
                    crc >>= 1;
                }
            }
        }
        crc
    }

    fn combine_bytes(msb: u8, lsb: u8) -> u16 {
        ((msb as u16) << 8) | lsb as u16
    }

    pub fn read(&mut self) -> Result<Measurement, Error> {
        // wake the AM2320 up, goes to sleep to not warm up and affect the humidity sensor
        // this write will fail as AM2320 won't ACK this write
        self.device
            .write(DEVICE_I2C_ADDR, &[0x00])
            .map_err(|e| Error::ReadError)?;
        // wait at least 0.8ms, at most 3ms
        self.delay.delay_us(1000);

        // send command
        // wait at least 1.5ms for the result
        self.device
            .write(DEVICE_I2C_ADDR, &[0x03, 0x00, 0x04])
            .map_err(|e| Error::ReadError)?;
        self.delay.delay_us(1600);

        // read out 8 bytes of result data
        // byte 0: Should be Modbus function code 0x03
        // byte 1: Should be number of registers to read (0x04)
        // byte 2: Humidity msb
        // byte 3: Humidity lsb
        // byte 4: Temperature msb
        // byte 5: Temperature lsb
        // byte 6: CRC lsb byte
        // byte 7: CRC msb byte
        let mut data: Vec<u8> = vec![0; 8];
        self.device
            .read(DEVICE_I2C_ADDR, &mut data)
            .map_err(|e| Error::ReadError)?;

        // check that the operation was reported as succesful
        if data[0] != 0x03 || data[1] != 0x04 {
            return Err(Error::SensorError);
        }

        // CRC check
        let crc = Self::crc16(&data[0..6]);
        if crc != Self::combine_bytes(data[7], data[6]) {
            return Err(Error::SensorError);
        }

        let temperature = Self::combine_bytes(data[4], data[5]);
        // TODO: fix this
        // if the highest bit is 1, the temperature is negative
        // if temperature & 0x8000 == temperature {
        //     temperature = -(temperature & 0x7FFF)
        // }
        let humidity = Self::combine_bytes(data[2], data[3]);

        Ok(Measurement {
            temperature: temperature as f64 / 10.0,
            humidity: humidity as f64 / 10.0,
        })
    }
}

#[test]
fn crc16() {
    assert_eq!(AM2320::crc16(&[]), 0xFFFF);
    assert_eq!(AM2320::crc16(&[0x03, 0x04, 0x02, 0x36, 0x0, 0xDB]), 0x0550);
}

#[test]
fn combine_bytes() {
    assert_eq!(AM2320::combine_bytes(0, 0), 0);
    assert_eq!(AM2320::combine_bytes(0xC5, 0x01), 0xDB);
}
