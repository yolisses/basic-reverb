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
        channels: u32,
        sample_rate: u32,
    ) -> Self {
        Self {
            basic_reverb: BasicReverb::new(
                room_size_ms,
                rt60,
                dry,
                wet,
                channels as usize,
                sample_rate,
            ),
        }
    }

    pub fn process_sample(&mut self, sample: f64) -> f64 {
        let channels = self.basic_reverb.channels;
        let resized_input = vec![sample; channels];
        let output = self.basic_reverb.process(resized_input);
        mean(&output)
    }
}
