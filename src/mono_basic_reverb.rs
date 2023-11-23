use crate::{basic_reverb::BasicReverb, constants::CHANNELS, mean::mean};

struct MonoBasicReverb {
    basic_reverb: BasicReverb,
}

impl MonoBasicReverb {
    pub(crate) fn new(room_size_ms: f64, rt60: f64, dry: f64, wet: f64) -> Self {
        Self {
            basic_reverb: BasicReverb::new(room_size_ms, rt60, dry, wet),
        }
    }

    fn process_sample(&mut self, sample: f64) -> f64 {
        let output = self.basic_reverb.process([sample; CHANNELS]);
        mean(&output)
    }
}
