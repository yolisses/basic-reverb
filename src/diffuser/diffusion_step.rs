use crate::array::Array;
use crate::constants::{CHANNELS, SAMPLE_RATE};
use crate::delay::Delay;
use crate::mix_matrix::hadmard::Hadamard;
use rand::Rng;

pub(crate) struct DiffusionStep {
    pub(crate) delay_ms_range: f64,
    delays: Vec<Delay>,
    flip_polarity: [bool; CHANNELS],
}

impl DiffusionStep {
    pub(crate) fn new() -> Self {
        let delay_ms_range = 50.;
        let delay_samples_range = delay_ms_range * 0.001 * SAMPLE_RATE;
        let mut delays = vec![];
        let mut flip_polarity = [false; CHANNELS];
        for i in 0..CHANNELS {
            let range_low: i64 = (delay_samples_range * i as f64 / CHANNELS as f64) as i64;
            let range_high: i64 = (delay_samples_range * (i as f64 + 1.) / CHANNELS as f64) as i64;

            let mut random = rand::thread_rng();

            let delay_size = random.gen_range(range_low..range_high);
            delays.push(Delay::new((delay_size + 1) as usize));

            let mut random = rand::thread_rng();
            flip_polarity[i] = random.gen_bool(0.5);
        }

        Self {
            delays,
            delay_ms_range,
            flip_polarity,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        // Delay
        let mut delayed = [0.; CHANNELS];
        for c in 0..CHANNELS {
            self.delays[c].write(input[c]);
            delayed[c] = self.delays[c].read();
        }

        // Mix with a Hadamard matrix
        let mut mixed = delayed;
        Hadamard::in_place(&mut mixed);

        // Flip some polarities
        for c in 0..CHANNELS {
            if self.flip_polarity[c] {
                mixed[c] *= -1.;
            }
        }

        return mixed;
    }
}
