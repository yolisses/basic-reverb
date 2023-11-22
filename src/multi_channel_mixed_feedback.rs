use crate::constants::CHANNELS;
use crate::delay::Delay;

pub(crate) struct MultiChannelMixedFeedback {
    delayMs: f64,
    decayGain: f64,
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
}
