#![allow(non_snake_case)]

mod strasen_OPTIMIEZED;
mod strasen_DEFOLT;

use rand::prelude::*;

pub use strasen_OPTIMIEZED::{razmer_lista, strasen_OPTIMIEZED};
pub use strasen_DEFOLT::strasen_DEFOLT;

pub fn classicMultiply(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    c.fill(0.0);
    for i in 0..n {
        let stroka_c = &mut c[i*n..(i + 1)*n];
        for k in 0..n{
            let aik = a[i * n + k];
            let rowB = &b[k*n..(k+1)*n];
            for j in 0..n {
                stroka_c[j] += aik * rowB[j];
            }
        }
    }
}

pub fn rand_matr(n: usize, seed: u64) -> Vec<f64> {
    let mut rnd = StdRng::seed_from_u64(seed);
    let mut matr = vec![0.0; n*n];
    for x in matr.iter_mut() {
        *x = rnd.random_range(-1.0..1.0);
    }
    return matr;
}

pub fn maxDiff(x: &[f64], y: &[f64]) -> f64 {
    let mut mx = 0.0;
    for i in 0..x.len() {
        let raznica = (x[i] - y[i]).abs();
        if raznica > mx { mx = raznica; }
    }
    mx
}
