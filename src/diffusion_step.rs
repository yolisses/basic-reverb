use crate::array::Array;
use crate::constants::CHANNELS;
use crate::delay::Delay;
use crate::hadmard::Hadamard;
use crate::sample_rate::SAMPLE_RATE;
use rand::Rng;

// TODO consider removing this derive
pub(crate) struct DiffusionStep {
    pub(crate) delay_ms_range: f64,
    delay_samples: [i64; CHANNELS],
    delays: Vec<Delay>,
    flip_polarity: [bool; CHANNELS],
}

impl DiffusionStep {
    pub(crate) fn new() -> Self {
        let delay_ms_range = 50.;
        let mut delays = vec![];
        let mut flip_polarity = [false; CHANNELS];
        let mut delay_samples = [0; CHANNELS];

        let delay_samples_range = delay_ms_range * 0.001 * SAMPLE_RATE;

        for i in 0..CHANNELS {
            // Adapt
            delays.push(Delay::new(0));

            let range_low: i64 = (delay_samples_range * i as f64 / CHANNELS as f64) as i64;
            let range_high: i64 = (delay_samples_range * (i as f64 + 1.) / CHANNELS as f64) as i64;

            let mut random = rand::thread_rng();

            delay_samples[i] = random.gen_range(range_low..range_high);
            delays[i].resize(delay_samples[i] + 1);
            delays[i].reset();

            let mut random = rand::thread_rng();
            flip_polarity[i] = random.gen_bool(0.5);
        }

        Self {
            delays,
            delay_ms_range,
            delay_samples,
            flip_polarity,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        // Delay
        let mut delayed = [0.; CHANNELS];
        for c in 0..CHANNELS {
            self.delays[c].write(input[c]);
            delayed[c] = self.delays[c].read(self.delay_samples[c]);
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
