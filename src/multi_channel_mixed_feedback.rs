use crate::array::Array;
use crate::constants::CHANNELS;
use crate::delay::Delay;
use crate::householder::Householder;
use crate::sample_rate::SAMPLE_RATE;

pub(crate) struct MultiChannelMixedFeedback {
    pub(crate) delayMs: f64,
    pub(crate) decayGain: f64,
    delaySamples: [i64; CHANNELS],
    delays: Vec<Delay>,
}

impl MultiChannelMixedFeedback {
    pub(crate) fn new() -> Self {
        let delayMs = 150.;
        let decayGain = 0.85;
        let mut delaySamples = [0; CHANNELS];
        let mut delays = vec![];

        // Adapt
        for i in 0..CHANNELS {
            delays.push(Delay::new(0));
        }

        let delaySamplesBase = delayMs * 0.001 * SAMPLE_RATE;
        for c in 0..CHANNELS {
            let r = c as f64 * 1.0 / CHANNELS as f64;
            delaySamples[c] = (f64::powf(2., r) * delaySamplesBase) as i64;
            delays[c].resize(delaySamples[c] + 1);
            delays[c].reset();
        }

        Self {
            delays,
            delayMs,
            decayGain,
            delaySamples,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        let mut delayed = [0.; CHANNELS];
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
