use crate::delay::Delay;
use crate::mix_matrix::householder::Householder;

pub(crate) struct MultiChannelMixedFeedback {
    pub(crate) decay_gain: f64,
    delays: Vec<Delay>,
}

impl MultiChannelMixedFeedback {
    pub(crate) fn new(delay_ms: f64, decay_gain: f64, channels: usize, sample_rate: u32) -> Self {
        let mut delays = Vec::with_capacity(channels);
        let delay_samples_base: f64 = delay_ms * 0.001 * sample_rate as f64;

        for i in 0..channels {
            let r = i as f64 * 1.0 / channels as f64;
            let delay_size = (f64::powf(2., r) * delay_samples_base) as usize;
            delays.push(Delay::new(delay_size + 1));
        }

        Self { delays, decay_gain }
    }

    pub(crate) fn process(&mut self, input: Vec<f64>) -> Vec<f64> {
        let channels = self.delays.len();
        let mut delayed = Vec::with_capacity(channels);

        for i in 0..channels {
            delayed.push(self.delays[i].read());
        }

        // Mix using a Householder matrix
        let mut mixed = delayed.clone();
        Householder::in_place(&mut mixed);

        for i in 0..channels {
            let sum = input[i] + mixed[i] * self.decay_gain;
            self.delays[i].write(sum);
        }

        delayed
    }
}
