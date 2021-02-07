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

    println!("---------------------------------------------------");
    let f = |x, m| x * m;
    let intensity: Array1<f64> = Array1::range(1., 6., 1.);
    let mean: Array1<f64> = Array1::range(1., 5., 1.);

    let num_elements = intensity.len() * mean.len();
    let mut v = Vec::with_capacity(num_elements);

    for x in intensity.iter() {
        for m in mean.iter() {
            v.push(f(*x, *m));
        }
    }
    dbg!(&v);

    let arr: Array2<f64> = Array::from_shape_vec((intensity.len(), mean.len()), v)
        .unwrap()
        .into();
    dbg!(&arr);
    println!("---------------------------------------------------");

    let f = |x, m, s| x * m * s;
    let intensity: Array1<f64> = Array1::range(1., 6., 1.);
    let mean: Array1<f64> = Array1::range(1., 5., 1.);
    let sd: Array1<f64> = Array1::range(1., 4., 1.);

    let num_elements = intensity.len() * mean.len() * sd.len();
    let mut v = Vec::with_capacity(num_elements);

    for x in intensity.iter() {
        for m in mean.iter() {
            for s in sd.iter() {
                v.push(f(*x, *m, *s));
            }
        }
    }
    dbg!(&v);

    let arr: Array3<f64> = Array::from_shape_vec((intensity.len(), mean.len(), sd.len()), v)
        .unwrap()
        .into();
    dbg!(&arr);
}
