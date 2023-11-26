use crate::delay::Delay;
use crate::mix_matrix::hadamard::Hadamard;
use rand::Rng;

pub(crate) struct DiffusionStep {
    pub(crate) delay_ms_range: f64,
    delays: Vec<Delay>,
    flip_polarity: Vec<bool>,
}

impl DiffusionStep {
    pub(crate) fn new(channels: usize, sample_rate: u32) -> Self {
        let delay_ms_range = 50.;
        let delay_samples_range = delay_ms_range * 0.001 * sample_rate as f64;

        let mut delays = Vec::with_capacity(channels);

        for i in 0..channels {
            let range_low: i64 = (delay_samples_range * i as f64 / channels as f64) as i64;
            let range_high: i64 = (delay_samples_range * (i as f64 + 1.) / channels as f64) as i64;

            let mut random = rand::thread_rng();

            let delay_size = random.gen_range(range_low..range_high);
            delays.push(Delay::new((delay_size + 1) as usize));
        }

        let mut random = rand::thread_rng();
        let flip_polarity = (0..channels).map(|_| random.gen_bool(0.5)).collect();

        Self {
            delays,
            delay_ms_range,
            flip_polarity,
        }
    }

    pub(crate) fn process(&mut self, input: Vec<f64>) -> Vec<f64> {
        // Delay
        let delayed: Vec<f64> = self
            .delays
            .iter_mut()
            .enumerate()
            .map(|(index, delay)| {
                delay.write(input[index]);
                delay.read()
            })
            .collect();

        // Mix with a Hadamard matrix
        let mut mixed = delayed;
        Hadamard::in_place(&mut mixed);

        // Flip some polarities
        for (index, flip) in self.flip_polarity.iter().enumerate() {
            if *flip {
                mixed[index] *= -1.;
            }
        }

        mixed
    }
}
