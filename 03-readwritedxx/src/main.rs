use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;
use std::fs::File;
use std::io::{Read, BufReader};
//use std::mem;
use std::vec::Vec;
use byteorder::{LittleEndian, WriteBytesExt};

fn main() {
    let name: &str = "/tmp/w5s.DSB";
    read_DSB(name).unwrap();
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
//    let mut buf = [0i16; mem::size_of::<i16>];
    let mut buf = [0; 2];
    loop {
        match reader.read(&mut buf)? {
            0 =>{
                break
            },
            2 => {
                let buf = &buf[..2];
                println!("{:?}", buf);
//                buf.as_mut
            }
            n => {
                //TODO
                println!("invalid bytes read: {} bytes", n);
            }
        }
    }
    let mut vec = Vec::new();
    vec.push(1.0f64);
    Ok(vec)
}


//fn read(filename: &String) -> Vec<f64> {}
