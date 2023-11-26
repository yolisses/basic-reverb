use crate::{basic_reverb::BasicReverb, mean::mean};

pub struct MonoBasicReverb {
    basic_reverb: BasicReverb,
}

impl MonoBasicReverb {
    pub fn new(
        room_size_ms: f64,
        rt60: f64,
        dry: f64,
        wet: f64,
        channels: usize,
        sample_rate: u32,
    ) -> Self {
        Self {
            basic_reverb: BasicReverb::new(room_size_ms, rt60, dry, wet, channels, sample_rate),
        }
    }

    pub fn process_sample(&mut self, sample: f64) -> f64 {
        let channels = self.basic_reverb.channels;
        let resized_input = Vec::with_capacity(channels);
        let output = self.basic_reverb.process(resized_input);
        mean(&output)
    }
}
