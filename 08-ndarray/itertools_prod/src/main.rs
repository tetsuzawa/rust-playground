use ndarray::prelude::*;
use itertools::iproduct;

fn main() {
    /*
    let intensity: Array1<f64> = Array1::range(1., 6., 1.);
    let mean: Array1<f64> = Array1::range(1., 5., 1.);
    // let sd: Array1<f64> = Array1::range(1., 4., 1.);
    let mut v = Vec::with_capacity(intensity.len() * mean.len());
    for (i, m) in iproduct!(intensity.iter(), mean.iter()) {
        v.push(i * m)
    };
    dbg!(&v);
    let arr = Array2::from_shape_vec((intensity.len(), mean.len()), v);
    dbg!(&arr);

    println!("-----------------------------------------------------------------------------------------------------------------------------------------------");

    let intensity: Array1<f64> = Array1::range(1., 6., 1.);
    let mean: Array1<f64> = Array1::range(1., 5., 1.);
    let sd: Array1<f64> = Array1::range(1., 4., 1.);
    let mut v = Vec::with_capacity(intensity.len() * mean.len() * sd.len());
    let source = vec![intensity.clone(),mean.clone(),sd.clone()];
    for (all, b,c) in iproduct!(source) {
        // v.push(i * m * s)
        for all2 in iproduct!(all){
            dbg!(all2);
        }
        // dbg!(all);
    };
    // for (i, m, s) in iproduct!(intensity.iter(), mean.iter(), sd.iter()) {
    //     v.push(i * m * s)
    // };
    v.push(1);
    dbg!(&v);
    let arr = Array::from_shape_vec((intensity.len(), mean.len(), sd.len()), v);
    dbg!(&arr);
    */
}

fn product_iter(src: &mut Vec<Array1<f64>>, piled: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    dbg!(&src);
    match src.pop() {
        Some(next) => {
            let mut piled_new = Vec::with_capacity(piled.len());
            if piled.is_empty() {
                for vv in next.iter() {
                    piled_new.push(vec![*vv])
                }
                product_iter(src, piled_new)
            } else {
                for (left, right) in iproduct!(next.iter(), piled.iter()) {
                    let mut rc = right.clone();
                    rc.push(*left);
                    piled_new.push(rc);
                };
                product_iter(src, piled_new)
            }
        }
        None => piled
    }
}

#[cfg(test)]
mod tests {
    use ndarray::prelude::*;
    use crate::product_iter;
    use itertools::iproduct;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_product() {
        let a = Array1::range(1., 5., 1.); //1,2,3,4
        let b = Array1::range(3., 6., 1.); //3,4,5
        let c = Array1::range(7., 9., 1.); //7,8
        let mut all = vec![a, b, c];
        // all.reverse();
        let res = product_iter(&mut all, vec![]);
        dbg!(res);
    }

    #[test]
    fn test_productiter() {
        let a = Array1::range(1., 5., 1.);
        // let b = Array1::range(3., 6., 1.);
        // let c = Array1::range(7., 9., 1.);
        // let mut all = vec![a, b, c];
        let mut v: Vec<Vec<f64>> = vec![];
        // all.reverse();
        println!("{:?}", v.is_empty());
        println!("{:?}", v.is_empty());
        if !v.is_empty() {
            println!("eeeeeee");
            for (l, r) in iproduct!(a.iter(), v.iter()) {
                dbg!(&l);
                dbg!(&r);
            }
        } else {
            println!("aaaa");
            // v.push(a.to_vec());
            for vv in a.iter() {
                v.push(vec![*vv])
            }
        };
        dbg!(v);
        // dbg!(&res);
    }
}
