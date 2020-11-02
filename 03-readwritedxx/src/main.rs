use std::str::FromStr;
use std::fmt;
use std::fs::File;
use std::mem;
use std::vec::Vec;
use std::path::Path;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::io::{Read, Cursor, BufReader, BufWriter};
use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

fn main() {
//    let name: &str = "sin440.DSB";
//    println!("{:?}", read_dsb(name).unwrap());
    println!("{:?}", read_dda("sin440.DDA").unwrap());
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

fn read_dfb(name: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
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

fn read_dsa(name: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let f = Box::new(BufReader::new(File::open(name)?));
    for result in f.lines() {
        let line = result?;
        let num: i16 = line.parse()?;
        data.push(num as f64);
    }
    Ok(data)
}

fn read_dfa(name: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let f = Box::new(BufReader::new(File::open(name)?));
    for result in f.lines() {
        let line = result?;
        let num: f32 = line.parse()?;
        data.push(num as f64);
    }
    Ok(data)
}

fn read_dda(name: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let f = Box::new(BufReader::new(File::open(name)?));
    for result in f.lines() {
        let line = result?;
        let num: f64 = line.parse()?;
        data.push(num);
    }
    Ok(data)
}


fn read(name: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let ext = match get_extension_from_filename(name) {
        Some(v) => Ok(v),
        None => Err(DXXError::InvalidSource)
    }?;
    let dtype = DTYPES::from_str(ext)?;
    match dtype {
        DTYPES::DSA => read_dsa(name),
        DTYPES::DFA => read_dfa(name),
        DTYPES::DDA => read_dda(name),
        DTYPES::DSB => read_dsb(name),
        DTYPES::DFB => read_dfb(name),
        DTYPES::DDB => read_ddb(name),
    }
}

fn get_extension_from_filename(name: &str) -> Option<&str> {
    Path::new(name)
        .extension()
        .and_then(OsStr::to_str)
}

//fn write_dsb(data: Vec<f64>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
//    let mut writer = BufWriter::new(File::create(name)?);
//    let mut buf = [0u8; mem::size_of::<i16>()];
//    let mut data = Vec::new();
//    loop {
//        match writer.read(&mut buf)? {
//            0 => {
//                break;
//            }
//            2 => {
//                let buf = &buf[..2];
//                let mut byte_reader = Cursor::new(buf);
//                let v = byte_reader.read_i16::<LittleEndian>()?;
//                data.push(v as f64);
//            }
//            _ => {
//                //TODO
//                continue;
//            }
//        }
//    }
//    Ok(data)
//}


trait NumVec {
    fn max<T>(data: &Vec<T>) -> T;
    fn min<T>(data: &Vec<T>) -> T;
}

//fn max<T>(data: &Vec<T>) -> Option<T> {
////    let mut max_val: T;
////    data.fold(0.0 / 0.0, |m, v| v.max(m))
//    data.iter().max().unwrap_or(T)
//}

fn f32s_to_int16s(data: &Vec<f32>) -> &Vec<i16> {
    let mut ret = Vec::new();
    let amp = i16::MAX - 1;
    let max_val = data.iter().max().unwrap_or(0f32);
    for v in data {
        ret.push(*v as f32)
    };
    &ret
}

fn f64s_to_int16s(data: &Vec<f64>) -> &Vec<i16> {
    let mut ret = Vec::new();
    for v in data {
        ret.push(*v as i16)
    };
    &ret
}

fn int16s_to_f32s(data: &Vec<i16>) -> &Vec<f32> {
    let mut ret = Vec::new();
    for v in data {
        ret.push(v as f32)
    };
    &ret
}
