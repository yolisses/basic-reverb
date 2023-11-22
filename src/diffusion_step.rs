use crate::constants::CHANNELS;
use crate::delay::Delay;
use crate::hadmard::Hadamard;
use crate::random_in_range::random_in_range;

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
            let rangeLow = delaySamplesRange * c as f64 / CHANNELS as f64;
            let rangeHigh = delaySamplesRange * (c as f64 + 1.) / CHANNELS as f64;
            self.delaySamples[c] = random_in_range(rangeLow, rangeHigh) as i64;
            self.delays[c].resize(self.delaySamples[c] + 1);
            self.delays[c].reset();
            self.flipPolarity[c] = random_bool();
        }
    }

    pub(crate) fn process(&mut self, input: [f64; CHANNELS]) -> [f64; CHANNELS] {
        // Delay
        let delayed = [0.; CHANNELS];
        for c in 0..CHANNELS {
            self.delays[c].write(input[c]);
            delayed[c] = self.delays[c].read(self.delaySamples[c]);
        }

        // Mix with a Hadamard matrix
        let mixed = delayed;
        Hadamard::inPlace(&mut mixed);

        // Flip some polarities
        for c in 0..CHANNELS {
            if (self.flipPolarity[c]) {
                mixed[c] *= -1.;
            }
        }

        return mixed;
    }
}
