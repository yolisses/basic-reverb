use crate::{basic_reverb::BasicReverb, mean::mean};

pub struct MonoBasicReverb<const CHANNELS: usize, const SAMPLE_RATE: usize> {
    basic_reverb: BasicReverb<CHANNELS, SAMPLE_RATE>,
}

impl<const CHANNELS: usize, const SAMPLE_RATE: usize> MonoBasicReverb<CHANNELS, SAMPLE_RATE> {
    pub fn new(room_size_ms: f64, rt60: f64, dry: f64, wet: f64) -> Self {
        Self {
            basic_reverb: BasicReverb::new(room_size_ms, rt60, dry, wet),
        }
    }

    pub fn process_sample(&mut self, sample: f64) -> f64 {
        let resized_input = [sample; CHANNELS];
        let output = self.basic_reverb.process(resized_input);
        mean(&output)
    }
}
