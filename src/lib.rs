use rppal::i2c::{self, I2c};

use std::{thread, time};

#[derive(Debug)]
pub enum CommunicationError {
    ReadingError,
    BusError(i2c::Error),
}

#[derive(Debug)]
pub struct AM2320 {
    pub temperature: f64,
    pub humidity: f64,
}

impl AM2320 {
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

    pub fn read() -> Result<Self, CommunicationError> {
        let mut device = I2c::new().map_err(|e| CommunicationError::BusError(e))?;
        device
            .set_slave_address(0x5c as u16)
            .map_err(|e| return CommunicationError::BusError(e))?;

        // wake AM2320 up, goes to sleep to not warm up and affect the humidity sensor
        // This write will fail as AM2320 won't ACK this write
        // Wait at least 0.8ms, at most 3ms
        device.write(&[0x00]).unwrap_or(0);
        thread::sleep(time::Duration::from_micros(1000));

        // send command
        // Wait at least 1.5ms for result
        device
            .write(&[0x03, 0x00, 0x04])
            .map_err(|_| CommunicationError::ReadingError)?;
        thread::sleep(time::Duration::from_micros(1600));

        // # Read out 8 bytes of result data
        // # Byte 0: Should be Modbus function code 0x03
        // # Byte 1: Should be number of registers to read (0x04)
        // # Byte 2: Humidity msb
        // # Byte 3: Humidity lsb
        // # Byte 4: Temperature msb
        // # Byte 5: Temperature lsb
        // # Byte 6: CRC lsb byte
        // # Byte 7: CRC msb byte
        let mut data: Vec<u8> = vec![0; 8];
        device.read(&mut data).unwrap();

        if data[0] != 0x03 || data[1] != 0x04 {
            return Err(CommunicationError::ReadingError);
        }

        // CRC check
        let crc = Self::crc16(&data[0..6]);
        if crc != Self::combine_bytes(data[7], data[6]) {
            return Err(CommunicationError::ReadingError);
        }

        let temperature = Self::combine_bytes(data[4], data[5]);
        // if the highest bit is 1, the temperature is negative
        // if temperature & 0x8000 == temperature {
        //     temperature = -(temperature & 0x7FFF)
        // }
        let humidity = Self::combine_bytes(data[2], data[3]);

        Ok(AM2320 {
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
    // assert_eq!(AM2320::combine_bytes(0xC5, 0x01), 0xDB);
}
