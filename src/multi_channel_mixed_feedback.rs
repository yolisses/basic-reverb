use crate::delay::Delay;
use crate::mix_matrix::householder::Householder;

pub(crate) struct MultiChannelMixedFeedback {
    pub(crate) decay_gain: f64,
    delays: Vec<Delay>,
    channels: usize,
}

impl MultiChannelMixedFeedback {
    pub(crate) fn new(delay_ms: f64, decay_gain: f64, channels: usize, sample_rate: u32) -> Self {
        let mut delays = vec![];

        let delay_samples_base: f64 = delay_ms * 0.001 * sample_rate as f64;
        for c in 0..channels {
            let r = c as f64 * 1.0 / channels as f64;
            let delay_size = (f64::powf(2., r) * delay_samples_base) as usize;
            delays.push(Delay::new(delay_size + 1));
        }

        Self {
            delays,
            channels,
            decay_gain,
        }
    }

    pub(crate) fn process(&mut self, input: Vec<f64>) -> Vec<f64> {
        let channels = self.channels;
        let mut delayed = Vec::with_capacity(channels);
        for c in 0..channels {
            delayed[c] = self.delays[c].read();
        }

        // Mix using a Householder matrix
        let mut mixed = delayed;
        Householder::in_place(&mut mixed);

        for c in 0..channels {
            let sum = input[c] + mixed[c] * self.decay_gain;
            self.delays[c].write(sum);
        }

        delayed
    }
}
