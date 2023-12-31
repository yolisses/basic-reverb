#[cfg(test)]
mod tests {
    use crate::mix_matrix::householder::Householder;

    #[test]
    fn it_works() {
        let mut data: [f64; 8] = [0., 0., 0., 0., 0., 0., 1., 0.];
        Householder::in_place(&mut data);
        assert_eq!(
            data,
            [-0.25, -0.25, -0.25, -0.25, -0.25, -0.25, 0.75, -0.25]
        );

        let mut data: [f64; 8] = [0., 0., 1., 0., 0., 0., 0., 0.];
        Householder::in_place(&mut data);
        assert_eq!(
            data,
            [-0.25, -0.25, 0.75, -0.25, -0.25, -0.25, -0.25, -0.25]
        );
    }
}
