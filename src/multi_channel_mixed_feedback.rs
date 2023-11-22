use crate::array::Array;
use crate::constants::CHANNELS;
use crate::delay::Delay;
use crate::householder::Householder;
use crate::sample_rate::SAMPLE_RATE;

pub(crate) struct MultiChannelMixedFeedback {
    pub(crate) delay_ms: f64,
    pub(crate) decay_gain: f64,
    delay_samples: [i64; CHANNELS],
    delays: Vec<Delay>,
}

impl MultiChannelMixedFeedback {
    pub(crate) fn new() -> Self {
        let delay_ms = 150.;
        let decay_gain = 0.85;
        let mut delay_samples = [0; CHANNELS];
        let mut delays = vec![];

        // Adapt
        for _i in 0..CHANNELS {
            delays.push(Delay::new(0));
        }

        let delay_samples_base = delay_ms * 0.001 * SAMPLE_RATE;
        for c in 0..CHANNELS {
            let r = c as f64 * 1.0 / CHANNELS as f64;
            delay_samples[c] = (f64::powf(2., r) * delay_samples_base) as i64;
            delays[c].resize(delay_samples[c] + 1);
            delays[c].reset();
        }

        Self {
            delays,
            delay_ms,
            decay_gain,
            delay_samples,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        let mut delayed = [0.; CHANNELS];
        for c in 0..CHANNELS {
            delayed[c] = self.delays[c].read(self.delay_samples[c]);
        }

        // Mix using a Householder matrix
        let mut mixed = delayed;
        Householder::inPlace(&mut mixed);

        for c in 0..CHANNELS {
            let sum = input[c] + mixed[c] * self.decay_gain;
            self.delays[c].write(sum);
        }

        return delayed;
    }
}
