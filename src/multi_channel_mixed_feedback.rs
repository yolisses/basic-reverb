use crate::array::Array;
use crate::constants::{CHANNELS, SAMPLE_RATE};
use crate::delay::Delay;
use crate::mix_matrix::householder::Householder;

pub(crate) struct MultiChannelMixedFeedback {
    pub(crate) delay_ms: f64,
    pub(crate) decay_gain: f64,
    delays: Vec<Delay>,
}

impl MultiChannelMixedFeedback {
    pub(crate) fn new() -> Self {
        let delay_ms = 150.;
        let decay_gain = 0.85;
        let mut delays = vec![];

        // Adapt
        for _i in 0..CHANNELS {}

        let delay_samples_base = delay_ms * 0.001 * SAMPLE_RATE;
        for c in 0..CHANNELS {
            let r = c as f64 * 1.0 / CHANNELS as f64;
            let delay_samples = (f64::powf(2., r) * delay_samples_base) as usize;
            delays.push(Delay::new(delay_samples + 1));
        }

        Self {
            delays,
            delay_ms,
            decay_gain,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        let mut delayed = [0.; CHANNELS];
        for c in 0..CHANNELS {
            delayed[c] = self.delays[c].read();
        }

        // Mix using a Householder matrix
        let mut mixed = delayed;
        Householder::in_place(&mut mixed);

        for c in 0..CHANNELS {
            let sum = input[c] + mixed[c] * self.decay_gain;
            self.delays[c].write(sum);
        }

        return delayed;
    }
}
