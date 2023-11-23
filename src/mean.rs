pub(crate) fn mean(values: &[f64]) -> f64 {
    let len = values.len();
    let mut sum = 0.;

    for c in 0..len {
        sum += values[c];
    }

    sum / len as f64
}
