#[cfg(test)]
mod tests {
    use crate::mix_matrix::create_hadamard_matrix::create_hadamard_matrix;

    #[test]
    fn with_size_4() {
        let hadamard_matrix = create_hadamard_matrix::<4>();
        assert_eq!(
            hadamard_matrix,
            [
                [1., 1., 1., 1.],
                [1., -1., 1., -1.],
                [1., 1., -1., -1.],
                [1., -1., -1., 1.],
            ]
        );
    }

    #[test]
    fn with_size_8() {
        let hadamard_matrix = create_hadamard_matrix::<8>();
        print!("{:?}", hadamard_matrix);
        assert_eq!(
            hadamard_matrix,
            [
                [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
                [1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0],
                [1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0],
                [1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0],
                [1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0],
                [1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0],
                [1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0]
            ]
        );
    }
}
