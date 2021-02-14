use basic_dsp::Convolution;
use basic_dsp::DspVec;
use std::vec::Vec;
use std::f64::consts::PI;


fn main() {
    let sampling_freq = 48000;
    let duration = &sampling_freq * 10;
    let sine = make_sine(duration, 440.0, sampling_freq);
    Convolution::convolve(sine)
    println!("{:?}", duration);
    println!("{:?}", sampling_freq);
    println!("{:?}", sine);
    println!("Hello, world!");
}

fn make_sine(length: i64, f: f64, sampling_freq: i64) -> Vec<f64> {
    let mut sine: DspVec = Vec::with_capacity(length as usize) as D;
    for i in 0..length {
        sine.push((2.0 * PI * f * i as f64 / sampling_freq as f64).sin())
    }
    return sine;
}
