use crate::classicMultiply;

pub fn razmer_lista(n: usize, porog: usize) -> (usize, usize) {
    let mut leaf = n;
    let mut kol = 1;
    while leaf > porog {
        leaf = leaf.div_ceil(2);
        kol *= 2;
    }
    (leaf, leaf*kol)
}

fn getIndex(i: usize, j: usize, sz: usize, leaf: usize) -> usize {
    if sz==leaf {
        return i * sz + j;
    }
    let pol = sz / 2;
    let chetvert = 2*(i / pol) + j/pol;
    chetvert * pol * pol + getIndex(i % pol, j % pol, pol, leaf)
}

pub fn v_bloki(a: &[f64], n: usize, sz: usize, leaf: usize) -> Vec<f64> {
    let mut bloki = vec![0.0; sz * sz];
    for i in 0..n {
        for j in (0..n).step_by(leaf) {
            let len = leaf.min(n - j);
            let k = getIndex(i, j, sz, leaf);
            bloki[k..k + len].copy_from_slice(&a[i*n + j..i*n + j + len]);
        }
    }
    bloki
}

pub fn izBlokov(bloki: &[f64], c: &mut [f64], n: usize, sz: usize, leaf: usize) {
    for i in 0..n {
        for j in (0..n).step_by(leaf){
            let len = leaf.min(n-j);
            let k = getIndex(i, j, sz, leaf);
            c[i * n + j..i * n + j + len].copy_from_slice(&bloki[k..k+len]);
        }
    }
}

fn summ(x: &[f64], y: &[f64], z: &mut [f64]) {
    for i in 0..z.len() {
        z[i] = x[i] + y[i];
    }
}

fn raznost(x: &[f64], y: &[f64], z: &mut [f64]) {
    for i in 0..z.len() {
        z[i] = x[i] - y[i];
    }
}

fn pribavit(z: &mut [f64], x: &[f64]) {
    for i in 0..z.len() {
        z[i] += x[i];
    }
}
fn sub_from(z: &mut [f64], x: &[f64])
{
    for i in 0..z.len() { z[i] -= x[i]; }
}

fn rec(a: &[f64], b: &[f64], c: &mut [f64], n: usize, leaf: usize, buf: &mut [f64]) {
    if n == leaf {
        classicMultiply(a, b, c, n);
        return;
    }

    let h = n/2;
    let q = h*h;
    let (a11, a12, a21, a22) = (&a[..q], &a[q..2 * q], &a[2 * q..3 * q], &a[3 * q..]);
    let (b11, b12, b21, b22) = (&b[..q], &b[q..2*q], &b[2*q..3*q], &b[3*q..]);
    let (c11, ost) = c.split_at_mut(q);
    let (c12, ost) = ost.split_at_mut(q);
    let (c21, c22) = ost.split_at_mut(q);
    let (tmp1, ost) = buf.split_at_mut(q);
    let (tmp2, ost) = ost.split_at_mut(q);
    let (M, buf2) = ost.split_at_mut(q);

    summ(a11, a22, tmp1);
    summ(b11, b22, tmp2);
    rec(tmp1, tmp2, c11, h, leaf, buf2);

    summ(a21, a22, tmp1);
    rec(tmp1, b11, c21, h, leaf, buf2);

    raznost(b12, b22, tmp2);
    rec(a11, tmp2, c12, h, leaf, buf2);

    raznost(c11, c21, c22);
    pribavit(c22, c12);

    raznost(b21, b11, tmp2);
    rec(a22, tmp2, M, h, leaf, buf2);
    pribavit(c11, M);
    pribavit(c21, M);


    summ(a11, a12, tmp1);
    rec(tmp1, b22, M, h, leaf, buf2);
    sub_from(c11, M);
    pribavit(c12, M);

    raznost(a21, a11, tmp1);
    summ(b11, b12, tmp2);
    rec(tmp1, tmp2, M, h, leaf, buf2);
    pribavit(c22, M);

    raznost(a12, a22, tmp1);
    summ(b21, b22, tmp2);
    rec(tmp1, tmp2, M, h, leaf, buf2);
    pribavit(c11, M);
}

pub fn strasen_OPTIMIEZED(a: &[f64], b: &[f64], c: &mut [f64], n: usize, porog: usize) {
    let (leaf, razmer) = razmer_lista(n, porog);
    let A = v_bloki(a, n, razmer, leaf);
    let B = v_bloki(b, n, razmer, leaf);
    let mut res = vec![0.0; razmer * razmer];
    let mut buffer = vec![0.0; razmer*razmer - leaf*leaf];
    rec(&A, &B, &mut res, razmer, leaf, &mut buffer);
    izBlokov(&res, c, n, razmer, leaf);
}
