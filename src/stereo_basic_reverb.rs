use crate::basic_reverb::BasicReverb;

pub struct StereoBasicReverb<const CHANNELS: usize, const SAMPLE_RATE: usize> {
    basic_reverb: BasicReverb<CHANNELS, SAMPLE_RATE>,
}

impl<const CHANNELS: usize, const SAMPLE_RATE: usize> StereoBasicReverb<CHANNELS, SAMPLE_RATE> {
    pub fn new(room_size_ms: f64, rt60: f64, dry: f64, wet: f64) -> Self {
        Self {
            basic_reverb: BasicReverb::new(room_size_ms, rt60, dry, wet),
        }
    }

    pub fn process_sample(&mut self, sample: (f64, f64)) -> (f64, f64) {
        let mut input = vec![0.; CHANNELS];

        // Duplicate input as many times as needed
        for i in 0..CHANNELS / 2 {
            input[2 * i] = sample.0;
            input[2 * i + 1] = sample.1;
        }

        let output = self.basic_reverb.process(input);

        // Mix down into stereo
        let mut sum = (0., 0.);
        for i in 0..CHANNELS / 2 {
            sum.0 = output[2 * i];
            sum.1 = output[2 * i + 1];
        }

        (sum.0 / (CHANNELS / 2) as f64, sum.1 / (CHANNELS / 2) as f64)
    }
}
