use crate::delay::Delay;
use crate::mix_matrix::householder::Householder;
use array_init::array_init;

pub(crate) struct MultiChannelMixedFeedback<const CHANNELS: usize, const SAMPLE_RATE: u32> {
    pub(crate) decay_gain: f64,
    delays: [Delay; CHANNELS],
}

impl<const CHANNELS: usize, const SAMPLE_RATE: u32>
    MultiChannelMixedFeedback<CHANNELS, SAMPLE_RATE>
{
    pub(crate) fn new(delay_ms: f64, decay_gain: f64) -> Self {
        let delay_samples_base: f64 = delay_ms * 0.001 * SAMPLE_RATE as f64;

        let delays = array_init(|i: usize| {
            let r = i as f64 * 1.0 / CHANNELS as f64;
            let delay_size = (f64::powf(2., r) * delay_samples_base) as usize;
            Delay::new(delay_size + 1)
        });

        Self { delays, decay_gain }
    }

    pub(crate) fn process(&mut self, input: [f64; CHANNELS]) -> [f64; CHANNELS] {
        let delayed = array_init(|i| self.delays[i].read());

        // Mix using a Householder matrix
        let mut mixed = delayed.clone();
        Householder::in_place(&mut mixed);

        for i in 0..CHANNELS {
            let sum = input[i] + mixed[i] * self.decay_gain;
            self.delays[i].write(sum);
        }

        delayed
    }
}
