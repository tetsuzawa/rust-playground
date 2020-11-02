use std::str::FromStr;
use std::fmt;
use std::fs::File;
use std::mem;
use std::vec::Vec;
use std::io::{Read, Cursor, BufReader};
use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let name: &str = "sin440.DSB";
    println!("{:?}", read_DSB(name).unwrap());
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DSA" => Ok(DTYPES::DSA),
            "DFA" => Ok(DTYPES::DFA),
            "DDA" => Ok(DTYPES::DDA),
            "DSB" => Ok(DTYPES::DSB),
            "DFB" => Ok(DTYPES::DFB),
            "DDB" => Ok(DTYPES::DDB),
            _ => Err(())
        }
    }
}


//fn style(name: &str) -> Result<DTYPES, Self::Err> {
//    type Err = ();
//    match DTYPES::from_str(name) {
//        Ok(dtype) => dtype,
//        Err(err) => err
//    }
//}

fn read_DSB(name: &str) -> Result<Vec<f64>, std::io::Error> {
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
                let v = byte_reader.read_i16::<LittleEndian>().unwrap();
                data.push(v as f64);
            }
            n => {
                //TODO
                println!("invalid number of bytes read: {} bytes", n);
            }
        }
    }
    Ok(data)
}


//fn read(filename: &String) -> Vec<f64> {}
