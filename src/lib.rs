//! A platform agnostic driver to interface with the 8 channel Relay (RS485)

use chrono;
use crc16::*;
use serialport::SerialPort;

#[derive(Debug)]
pub enum Error {
    SerialError(serialport::Error),
    IoError(std::io::Error),
    DecodeError,
    ResponseError,
    StatusError,
    OpFailed,
}

pub enum ChannelNum {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Clone, Copy, Debug)]
pub enum SwitchStatus {
    On,
    Off,
}
pub struct Relay {
    modbus_addr: u8,
    port: Box<dyn SerialPort>,
}

impl Relay {
    pub fn new(
        port_path: &str,
        modbus_addr: u8,
        baud_rate: u32,
    ) -> Result<Self, serialport::Error> {
        let mut port = serialport::open(port_path)?;
        port.set_baud_rate(baud_rate)?;
        Ok(Relay {
            // port_path,
            modbus_addr,
            // baud_rate,
            port,
        })
    }

    pub fn get_reg_all(&mut self) -> Result<[u8; 2], Error> {
        let mut buffer: Vec<u8> = vec![self.modbus_addr, 0x03, 0x00, 0x00, 0x00, 0x01];
        let crc = crc16::State::<MODBUS>::calculate(&buffer).to_le_bytes();
        buffer.extend_from_slice(&crc);
        self.port.write(&buffer).map_err(|e| Error::IoError(e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        let mut buf = [0_u8; 1024];
        let n = self.port.read(&mut buf).map_err(|e| Error::IoError(e))?;

        // DEBUG:
        // println!("receive: {:?}", &buf[..n]);

        if n != 7 {
            return Err(Error::DecodeError);
        }

        let crc = crc16::State::<MODBUS>::calculate(&buf[..n - 2]);
        let crc_check = crc == u16::from_le_bytes([buf[n - 2], buf[n - 1]]);
        if !crc_check {
            eprintln!(
                "{} crc check error for reply from hwt905",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
            return Err(Error::DecodeError);
        }

        Ok([buf[3], buf[4]])
    }

    pub fn get_coils(&mut self) -> Result<[SwitchStatus; 8], Error> {
        let mut buffer: Vec<u8> = vec![self.modbus_addr, 0x01, 0x00, 0x00, 0x00, 0x08];
        let crc = crc16::State::<MODBUS>::calculate(&buffer).to_le_bytes();
        buffer.extend_from_slice(&crc);
        self.port.write(&buffer).map_err(|e| Error::IoError(e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        let mut buf = [0_u8; 1024];
        let n = self.port.read(&mut buf).map_err(|e| Error::IoError(e))?;

        // DEBUG:
        // println!("receive: {:?}", &buf[..n]);

        if n != 6 {
            return Err(Error::DecodeError);
        }

        let crc = crc16::State::<MODBUS>::calculate(&buf[..n - 2]);
        let crc_check = crc == u16::from_le_bytes([buf[n - 2], buf[n - 1]]);
        if !crc_check {
            eprintln!(
                "{} crc check error for reply from hwt905",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
            return Err(Error::DecodeError);
        }

        let mut status = [SwitchStatus::Off; 8];
        let status_code = buf[3];
        for i in 0..8 {
            match status_code & 1 << i {
                0 => status[i as usize] = SwitchStatus::Off,
                _ => status[i as usize] = SwitchStatus::On,
            }
        }
        Ok(status)
    }

    pub fn off(&mut self, num: ChannelNum) -> Result<(), Error> {
        let n: u8 = match num {
            ChannelNum::One => 0,
            ChannelNum::Two => 1,
            ChannelNum::Three => 2,
            ChannelNum::Four => 3,
            ChannelNum::Five => 4,
            ChannelNum::Six => 5,
            ChannelNum::Seven => 6,
            ChannelNum::Eight => 7,
        };
        let mut buffer: Vec<u8> = vec![self.modbus_addr, 0x05, 0x00, n, 0x00, 0x00];
        let crc = crc16::State::<MODBUS>::calculate(&buffer).to_le_bytes();
        buffer.extend_from_slice(&crc);
        self.port.write(&buffer).map_err(|e| Error::IoError(e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        let mut buf = [0_u8; 1024];
        let n = self.port.read(&mut buf).map_err(|e| Error::IoError(e))?;

        // DEBUG:
        // println!("receive: {:?}", &buf[..n]);

        if n != 8 {
            return Err(Error::ResponseError);
        }

        let crc = crc16::State::<MODBUS>::calculate(&buf[..n - 2]);
        let crc_check = crc == u16::from_le_bytes([buf[n - 2], buf[n - 1]]);
        if !crc_check {
            eprintln!(
                "{} crc check error for reply from hwt905",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
            return Err(Error::DecodeError);
        }

        match buf[4..6] {
            [0x00, 0x00] => Ok(()),
            _ => Err(Error::OpFailed),
        }
    }

    pub fn on(&mut self, num: ChannelNum) -> Result<(), Error> {
        let n: u8 = match num {
            ChannelNum::One => 0,
            ChannelNum::Two => 1,
            ChannelNum::Three => 2,
            ChannelNum::Four => 3,
            ChannelNum::Five => 4,
            ChannelNum::Six => 5,
            ChannelNum::Seven => 6,
            ChannelNum::Eight => 7,
        };
        let mut buffer: Vec<u8> = vec![self.modbus_addr, 0x05, 0x00, n, 0xFF, 0x00];
        let crc = crc16::State::<MODBUS>::calculate(&buffer).to_le_bytes();
        buffer.extend_from_slice(&crc);
        self.port.write(&buffer).map_err(|e| Error::IoError(e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        let mut buf = [0_u8; 1024];
        let n = self.port.read(&mut buf).map_err(|e| Error::IoError(e))?;

        // DEBUG:
        // println!("receive: {:?}", &buf[..n]);

        if n != 8 {
            return Err(Error::ResponseError);
        }

        let crc = crc16::State::<MODBUS>::calculate(&buf[..n - 2]);
        let crc_check = crc == u16::from_le_bytes([buf[n - 2], buf[n - 1]]);
        if !crc_check {
            eprintln!(
                "{} crc check error for reply from hwt905",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
            return Err(Error::DecodeError);
        }

        match buf[4..6] {
            [0xFF, 0x00] => Ok(()),
            _ => Err(Error::OpFailed),
        }
    }

    pub fn off_reg(&mut self, num: ChannelNum) -> Result<(), Error> {
        let status = self.get_reg_all()?;
        if status[0] != 0 {
            return Err(Error::StatusError);
        }
        let n: u8 = match num {
            ChannelNum::One => 0b11111110,
            ChannelNum::Two => 0b11111101,
            ChannelNum::Three => 0b11111011,
            ChannelNum::Four => 0b11110111,
            ChannelNum::Five => 0b11101111,
            ChannelNum::Six => 0b11011111,
            ChannelNum::Seven => 0b10111111,
            ChannelNum::Eight => 0b01111111,
        };
        let mut buffer: Vec<u8> = vec![self.modbus_addr, 0x06, 0x00, 0x00, 0x00, status[1] & n];
        let crc = crc16::State::<MODBUS>::calculate(&buffer).to_le_bytes();
        buffer.extend_from_slice(&crc);
        self.port.write(&buffer).map_err(|e| Error::IoError(e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        let mut buf = [0_u8; 1024];
        let n = self.port.read(&mut buf).map_err(|e| Error::IoError(e))?;

        // DEBUG:
        // println!("receive: {:?}", &buf[..n]);

        if n != 8 {
            return Err(Error::ResponseError);
        }

        let crc = crc16::State::<MODBUS>::calculate(&buf[..n - 2]);
        let crc_check = crc == u16::from_le_bytes([buf[n - 2], buf[n - 1]]);
        if !crc_check {
            eprintln!(
                "{} crc check error for reply from hwt905",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
            return Err(Error::DecodeError);
        }

        Ok(())
    }

    pub fn on_reg(&mut self, num: ChannelNum) -> Result<(), Error> {
        let status = self.get_reg_all()?;
        if status[0] != 0 {
            return Err(Error::StatusError);
        }
        let n: u8 = match num {
            ChannelNum::One => 0b00000001,
            ChannelNum::Two => 0b00000010,
            ChannelNum::Three => 0b00000100,
            ChannelNum::Four => 0b00001000,
            ChannelNum::Five => 0b00010000,
            ChannelNum::Six => 0b00100000,
            ChannelNum::Seven => 0b01000000,
            ChannelNum::Eight => 0b10000000,
        };
        let mut buffer: Vec<u8> = vec![self.modbus_addr, 0x06, 0x00, 0x00, 0x00, status[1] | n];
        let crc = crc16::State::<MODBUS>::calculate(&buffer).to_le_bytes();
        buffer.extend_from_slice(&crc);
        self.port.write(&buffer).map_err(|e| Error::IoError(e))?;

        std::thread::sleep(std::time::Duration::from_millis(50));

        let mut buf = [0_u8; 1024];
        let n = self.port.read(&mut buf).map_err(|e| Error::IoError(e))?;

        // DEBUG:
        // println!("receive: {:?}", &buf[..n]);

        if n != 8 {
            return Err(Error::ResponseError);
        }

        let crc = crc16::State::<MODBUS>::calculate(&buf[..n - 2]);
        let crc_check = crc == u16::from_le_bytes([buf[n - 2], buf[n - 1]]);
        if !crc_check {
            eprintln!(
                "{} crc check error for reply from hwt905",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
            return Err(Error::DecodeError);
        }

        Ok(())
    }
}
