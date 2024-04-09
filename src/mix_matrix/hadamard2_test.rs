#[cfg(test)]
mod tests {
    use crate::mix_matrix::hadamard2::Hadamard2;

    #[test]
    fn zeros() {
        let hadamard2 = Hadamard2::<8>::new();
        let data: [f64; 8] = [0., 0., 0., 0., 0., 0., 0., 0.];
        let result = hadamard2.process(data);
        assert_eq!(result, [0., 0., 0., 0., 0., 0., 0., 0.]);
    }

    #[test]
    fn one() {
        let hadamard2 = Hadamard2::<8>::new();
        let data: [f64; 8] = [0., 0., 0., 0., 0., 0., 1., 0.];
        let result = hadamard2.process(data);
        assert_eq!(result, [1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0]);
    }
}
