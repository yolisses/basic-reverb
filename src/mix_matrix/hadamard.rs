pub struct Hadamard {}

// Use like `Hadamard::inPlace(data)`
// data.len() must be a power of 2
impl Hadamard {
    fn recursive_unscaled(data: &mut [f64]) {
        let size = data.len();
        if size <= 1 {
            return;
        }

        let half_size = size / 2;

        // Two (unscaled) Hadamards of half the size
        Hadamard::recursive_unscaled(&mut data[0..half_size]);
        Hadamard::recursive_unscaled(&mut data[half_size..]);

        // Combine the two halves using sum/difference
        for i in 0..half_size {
            let a = data[i];
            let b = data[i + half_size];
            data[i] = a + b;
            data[i + half_size] = a - b;
        }
    }

    pub fn in_place(data: &mut [f64]) {
        let size = data.len();
        Hadamard::recursive_unscaled(data);

        let scaling_factor = (1.0 / size as f64).sqrt();
        for i in 0..size {
            data[i] *= scaling_factor;
        }
    }
}
