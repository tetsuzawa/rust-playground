use std::str::FromStr;
use std::fmt;
use std::fs::File;
use std::mem;
use std::vec::Vec;
use std::io::{Read, Cursor, BufReader};
use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

fn main() {
    let name: &str = "sin440.DSB";
    println!("{:?}", read_dsb(name).unwrap());
}

enum DTYPES {
    DSA,
    DFA,
    DDA,
    DSB,
    DFB,
    DDB,
}

impl fmt::Display for DTYPES {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DTYPES::DSA => write!(f, "DSA"),
            DTYPES::DFA => write!(f, "DFA"),
            DTYPES::DDA => write!(f, "DDA"),
            DTYPES::DSB => write!(f, "DSB"),
            DTYPES::DFB => write!(f, "DFB"),
            DTYPES::DDB => write!(f, "DDB"),
        }
    }
}

impl FromStr for DTYPES {
    type Err = DXXError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DSA" => Ok(DTYPES::DSA),
            "DFA" => Ok(DTYPES::DFA),
            "DDA" => Ok(DTYPES::DDA),
            "DSB" => Ok(DTYPES::DSB),
            "DFB" => Ok(DTYPES::DFB),
            "DDB" => Ok(DTYPES::DDB),
            _ => Err(DXXError::InvalidSource),
        }
    }
}

#[derive(Debug, Error)]
pub enum DXXError {
    #[error("Invalid source")]
    InvalidSource,
}


fn style(name: &str) -> Result<DTYPES, Box<dyn std::error::Error>> {
    Ok(DTYPES::from_str(name)?)
}

fn read_dsb(name: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(File::open(name)?);
    let mut buf = [0u8; mem::size_of::<i16>()];
    let mut data = Vec::new();
    loop {
        match reader.read(&mut buf)? {
            0 => {
                break;
            }
            2 => {
                let buf = &buf[..2];
                let mut byte_reader = Cursor::new(buf);
                let v = byte_reader.read_i16::<LittleEndian>()?;
                data.push(v as f64);
            }
            _ => {
                //TODO
                continue;
            }
        }
    }
    Ok(data)
}

fn read_dfb(name: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    const BYTE_SIZE: usize = mem::size_of::<f32>();
    let mut reader = BufReader::new(File::open(name)?);
    let mut buf = [0u8; BYTE_SIZE];
    let mut data = Vec::new();
    loop {
        match reader.read(&mut buf)? {
            0 => {
                break;
            }
            BYTE_SIZE => {
                let buf = &buf[..BYTE_SIZE];
                let mut byte_reader = Cursor::new(buf);
                let v = byte_reader.read_f32::<LittleEndian>()?;
                data.push(v);
            }
            _ => {
                //TODO
                continue;
            }
        }
    }
    Ok(data)
}

fn read_ddb(name: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    const BYTE_SIZE: usize = mem::size_of::<f64>();
    let mut reader = BufReader::new(File::open(name)?);
    let mut buf = [0u8; BYTE_SIZE];
    let mut data = Vec::new();
    loop {
        match reader.read(&mut buf)? {
            0 => {
                break;
            }
            BYTE_SIZE => {
                let buf = &buf[..BYTE_SIZE];
                let mut byte_reader = Cursor::new(buf);
                let v = byte_reader.read_f64::<LittleEndian>()?;
                data.push(v);
            }
            _ => {
                //TODO
                continue;
            }
        }
    }
    Ok(data)
}
//fn read(filename: &String) -> Vec<f64> {}
