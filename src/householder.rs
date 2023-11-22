pub(crate) struct Householder {}

// Use like `Householder::inPlace(data)`
// data.len() must be ≥ 1
impl Householder {
    pub(crate) fn in_place(data: &mut [f64]) {
        let size = data.len();
        let multiplier = -2. / size as f64;

        let mut sum = 0.;
        for i in 0..size {
            sum += data[i]
        }

        sum *= multiplier;

        for i in 0..size {
            data[i] += sum;
        }
    }
}
