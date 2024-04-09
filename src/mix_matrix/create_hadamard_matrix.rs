pub(crate) fn create_hadamard_matrix<const N: usize>() -> [[f64; N]; N] {
    // Initializing a matrix of order N
    let mut matrix = [[0.; N]; N];

    // Initializing the 0th column and 0th row element as 1
    matrix[0][0] = 1.;

    let mut k = 1;
    while k < N {
        // Loop to copy elements to other quarters of the matrix
        for i in 0..k {
            for j in 0..k {
                matrix[i + k][j] = matrix[i][j];
                matrix[i][j + k] = matrix[i][j];
                matrix[i + k][j + k] = -matrix[i][j];
            }
        }
        k += k;
    }

    matrix
}
