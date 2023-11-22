use crate::constants::CHANNELS;
use crate::delay::Delay;

// TODO
fn random_bool() -> bool {
    true
}

pub(crate) struct DiffusionStep {
    delayMsRange: f64,
    delaySamples: [i64; CHANNELS],
    delays: [Delay; CHANNELS],
    flipPolarity: [bool; CHANNELS],
}

impl DiffusionStep {
    pub(crate) fn configure(&mut self, sampleRate: f64) {
        let delaySamplesRange = self.delayMsRange * 0.001 * sampleRate;
        for c in 0..CHANNELS {
            let rangeLow = delaySamplesRange * c / CHANNELS;
            let rangeHigh = delaySamplesRange * (c + 1) / CHANNELS;
            self.delaySamples[c] = randomInRange(rangeLow, rangeHigh);
            self.delays[c].resize(self.delaySamples[c] + 1);
            Delay::reset();
            self.flipPolarity[c] = random_bool();
        }
    }
}
