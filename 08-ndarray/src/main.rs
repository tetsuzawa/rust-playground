use ndarray::prelude::*;
use ndarray::OwnedRepr;
use std::borrow::Borrow;

fn double(v: f64) -> f64 {
    v * 2.0
}

fn do_smth(x: usize, y: usize, z: usize, value: f64) {
    println!("{}, {}, {}, {}", x, y, z, value);
}

fn main() {
    let arr = Array::range(0., 60., 1.).to_vec();
    dbg!(&arr);
    let mut arr = Array::from_shape_vec((3, 4, 5), arr).unwrap();
    dbg!(&arr);
    // for ((x, y, z), value) in arr.indexed_iter() {
    //     do_smth(x, y, z, *value);
    // }
    for value in arr.iter_mut() {
        *value = double(*value)
    }
    dbg!(&arr);
    println!("Hello, world!");
}
