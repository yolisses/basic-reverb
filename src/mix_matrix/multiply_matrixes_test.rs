#[cfg(test)]
mod tests {
    use crate::mix_matrix::multiply_matrixes::multiply_matrixes;

    #[test]
    fn cell() {
        // 1
        let matrix_a = [[1.]];

        // 2
        let matrix_b = [[2.]];

        // 2
        let matrix_c = [[2.]];

        let result = multiply_matrixes(matrix_a, matrix_b);
        assert_eq!(result, matrix_c);
    }

    #[test]
    fn line() {
        // 1 2
        let matrix_a = [[1., 2.]];

        // 3
        // 4
        let matrix_b = [[3.], [4.]];

        // 11
        let matrix_c = [[11.]];

        let result = multiply_matrixes(matrix_a, matrix_b);
        assert_eq!(result, matrix_c);
    }

    #[test]
    fn simple() {
        // 1 2 3
        // 4 5 6
        let matrix_a = [[1., 2., 3.], [4., 5., 6.]];

        // 7  8
        // 9  10
        // 11 11
        let matrix_b = [[7., 8.], [9., 10.], [11., 12.]];

        // 58  64
        // 139 154
        let matrix_c = [[58., 64.], [139., 154.]];

        let result = multiply_matrixes(matrix_a, matrix_b);
        assert_eq!(result, matrix_c);
    }
}
