use super::{create_hadamard_matrix::create_hadamard_matrix, multiply_matrixes::multiply_matrixes};

pub struct Hadamard2<const SIZE: usize> {
    matrix: [[f64; SIZE]; SIZE],
}

// Use like `Hadamard::inPlace(data)`
// data.len() must be a power of 2
impl<const SIZE: usize> Hadamard2<SIZE> {
    pub(crate) fn new() -> Self {
        Self {
            matrix: create_hadamard_matrix(),
        }
    }

    pub unsafe fn process(&self, data: [f64; SIZE]) -> [f64; SIZE] {
        let result = multiply_matrixes([data], self.matrix);
        result[0]
    }
}
