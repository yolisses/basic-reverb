use crate::array::Array;
use crate::constants::CHANNELS;
use crate::delay::Delay;
use crate::hadmard::Hadamard;
use crate::random_in_range::random_in_range;
use crate::sample_rate::SAMPLE_RATE;

// TODO
fn random_bool() -> bool {
    true
}

// TODO consider removing this derive
pub(crate) struct DiffusionStep {
    pub(crate) delayMsRange: f64,
    delaySamples: [i64; CHANNELS],
    delays: Vec<Delay>,
    flipPolarity: [bool; CHANNELS],
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

            let range_low = delay_samples_range * i as f64 / CHANNELS as f64;
            let range_high = delay_samples_range * (i as f64 + 1.) / CHANNELS as f64;
            delay_samples[i] = random_in_range(range_low, range_high) as i64;
            delays[i].resize(delay_samples[i] + 1);
            delays[i].reset();
            flip_polarity[i] = random_bool();
        }

        Self {
            delays,
            delayMsRange: delay_ms_range,
            delaySamples: delay_samples,
            flipPolarity: flip_polarity,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        // Delay
        let mut delayed = [0.; CHANNELS];
        for c in 0..CHANNELS {
            self.delays[c].write(input[c]);
            delayed[c] = self.delays[c].read(self.delaySamples[c]);
        }

        // Mix with a Hadamard matrix
        let mut mixed = delayed;
        Hadamard::inPlace(&mut mixed);

        // Flip some polarities
        for c in 0..CHANNELS {
            if self.flipPolarity[c] {
                mixed[c] *= -1.;
            }
        }

        return mixed;
    }
}
