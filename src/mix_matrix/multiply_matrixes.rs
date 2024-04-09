use matrixmultiply::dgemm;

pub(crate) fn multiply_matrixes<const M: usize, const K: usize, const N: usize>(
    matrix_a: [[f64; K]; M], // M x K
    matrix_b: [[f64; N]; K], // K x N
) -> [[f64; N]; M] {
    let mut matrix_c = [[0.; N]; M]; // M x N

    let m: usize = M;
    let k: usize = K;
    let n: usize = N;

    let a: *const f64 = matrix_a[0].as_ptr();
    let rsa: isize = k as isize;
    let csa: isize = 1;

    let b = matrix_b[0].as_ptr();
    let rsb = n as isize;
    let csb = 1;

    let c = matrix_c[0].as_mut_ptr();
    let rsc = n as isize;
    let csc = 1;

    let alpha: f64 = 1.;
    let beta = 0.;

    unsafe { dgemm(m, k, n, alpha, a, rsa, csa, b, rsb, csb, beta, c, rsc, csc) };

    matrix_c
}
