use crate::classicMultiply;
use crate::strasen_OPTIMIEZED::{izBlokov, razmer_lista, v_bloki};

fn plus(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y).map(|(x, y)| x + y).collect()
}

fn minus(x: &[f64], y: &[f64]) -> Vec<f64> {
    x.iter().zip(y).map(|(x, y)| x - y).collect()
}

fn strasen_rekurs(a: &[f64], b: &[f64], n: usize, list: usize) -> Vec<f64> {
    if n==list {
        let mut c = vec![0.0; n * n];
        classicMultiply(a, b, &mut c, n);
        return c;
    }

    let half = n / 2;
    let q = half*half;
    let (a11, a12, a21, a22) = (&a[..q], &a[q..2 * q], &a[2 * q..3 * q], &a[3 * q..]);
    let (b11, b12, b21, b22) = (&b[..q], &b[q..2 * q], &b[2 * q..3 * q], &b[3 * q..]);

    let m1 = strasen_rekurs(&plus(a11, a22), &plus(b11, b22), half, list);
    let m2 = strasen_rekurs(&plus(a21, a22), b11, half, list);
    let m3 = strasen_rekurs(a11, &minus(b12, b22), half, list);
    let m4 = strasen_rekurs(a22, &minus(b21, b11), half, list);
    let m5 = strasen_rekurs(&plus(a11, a12), b22, half, list);
    let m6 = strasen_rekurs(&minus(a21, a11), &plus(b11, b12), half, list);
    let m7 = strasen_rekurs(&minus(a12, a22), &plus(b21, b22), half, list);

    let c11 = plus(&minus(&plus(&m1, &m4), &m5), &m7);
    let c12 = plus(&m3, &m5);
    let c21 = plus(&m2, &m4);
    let c22 = plus(&plus(&minus(&m1, &m2), &m3), &m6);
    [c11, c12, c21, c22].concat()
}

pub fn strasen_DEFOLT(a: &[f64], b: &[f64], c: &mut [f64], n: usize, t: usize) {
    let (list, size) = razmer_lista(n, t);
    let a2 = v_bloki(a, n, size, list);
    let b2 = v_bloki(b, n, size, list);
    let rez = strasen_rekurs(&a2, &b2, size, list);
    izBlokov(&rez, c, n, size, list);
}
