use crate::array::Array;
use crate::constants::CHANNELS;
use crate::delay::Delay;
use crate::hadmard::Hadamard;
use crate::random_in_range::random_in_range;
use crate::sample_rate::SAMPLE_RATE;

// TODO
fn random_bool() -> bool {
    true
}

// TODO consider removing this derive
pub(crate) struct DiffusionStep {
    pub(crate) delayMsRange: f64,
    delaySamples: [i64; CHANNELS],
    delays: Vec<Delay>,
    flipPolarity: [bool; CHANNELS],
}

impl DiffusionStep {
    pub(crate) fn new() -> Self {
        let delayMsRange = 50.;
        let mut delays = vec![];
        let mut flipPolarity = [false; CHANNELS];
        let mut delaySamples = [0; CHANNELS];

        let delaySamplesRange = delayMsRange * 0.001 * SAMPLE_RATE;

        for i in 0..CHANNELS {
            // Adapt
            delays.push(Delay::new(0));

            let rangeLow = delaySamplesRange * i as f64 / CHANNELS as f64;
            let rangeHigh = delaySamplesRange * (i as f64 + 1.) / CHANNELS as f64;
            delaySamples[i] = random_in_range(rangeLow, rangeHigh) as i64;
            delays[i].resize(delaySamples[i] + 1);
            delays[i].reset();
            flipPolarity[i] = random_bool();
        }

        Self {
            delays,
            delayMsRange,
            delaySamples,
            flipPolarity,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        // Delay
        let mut delayed = [0.; CHANNELS];
        for c in 0..CHANNELS {
            self.delays[c].write(input[c]);
            delayed[c] = self.delays[c].read(self.delaySamples[c]);
        }

        // Mix with a Hadamard matrix
        let mut mixed = delayed;
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
