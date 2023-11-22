use crate::array::Array;
use crate::constants::CHANNELS;
use crate::delay::Delay;
use crate::householder::Householder;

pub(crate) struct MultiChannelMixedFeedback {
    pub(crate) delayMs: f64,
    pub(crate) decayGain: f64,
    delaySamples: [i64; CHANNELS],
    delays: [Delay; CHANNELS],
}

impl MultiChannelMixedFeedback {
    pub(crate) fn configure(&mut self, sampleRate: f64) {
        let delaySamplesBase = self.delayMs * 0.001 * sampleRate;
        for c in 0..CHANNELS {
            let r = c as f64 * 1.0 / CHANNELS as f64;
            self.delaySamples[c] = (f64::powf(2., r) * delaySamplesBase) as i64;
            self.delays[c].resize(self.delaySamples[c] + 1);
            self.delays[c].reset();
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        let mut delayed: Array;
        for c in 0..CHANNELS {
            delayed[c] = self.delays[c].read(self.delaySamples[c]);
        }

        // Mix using a Householder matrix
        let mut mixed = delayed;
        Householder::inPlace(&mut mixed);

        for c in 0..CHANNELS {
            let sum = input[c] + mixed[c] * self.decayGain;
            self.delays[c].write(sum);
        }

        return delayed;
    }
}

// new
impl MultiChannelMixedFeedback {
    pub(crate) fn new() -> Self {
        Self {
            delayMs: 150.,
            decayGain: 0.85,
            delaySamples: [0; CHANNELS],
            delays: [Delay::new(); CHANNELS],
        }
    }
}
