use crate::diffuser::diffuser_half_lengths::DiffuserHalfLengths;
use crate::multi_channel_mixed_feedback::MultiChannelMixedFeedback;

pub struct BasicReverb {
    dry: f64,
    wet: f64,
    diffuser: DiffuserHalfLengths,
    feedback: MultiChannelMixedFeedback,
    pub(crate) channels: usize,
}

impl BasicReverb {
    pub fn new(
        room_size_ms: f64,
        rt60: f64,
        dry: f64,
        wet: f64,
        channels: usize,
        sample_rate: u32,
    ) -> Self {
        let diffuser = DiffuserHalfLengths::new(room_size_ms, channels, sample_rate);

        // How long does our signal take to go around the feedback loop?
        let typical_loop_ms = room_size_ms * 1.5;
        // How many times will it do that during our RT60 period?
        let loops_per_rt_60 = rt60 / (typical_loop_ms * 0.001);
        // This tells us how many dB to reduce per loop
        let db_per_cycle = -60. / loops_per_rt_60;

        let delay_ms = room_size_ms;
        let decay_gain = f64::powf(10., db_per_cycle * 0.05);
        let feedback = MultiChannelMixedFeedback::new(delay_ms, decay_gain, channels, sample_rate);

        Self {
            dry,
            wet,
            channels,
            feedback,
            diffuser,
        }
    }

    pub fn process(&mut self, input: Vec<f64>) -> Vec<f64> {
        let channels = self.channels;
        let diffuse = self.diffuser.process(input.clone());
        let long_lasting = self.feedback.process(diffuse);

        let mut output = vec![];
        for i in 0..channels {
            let dry = self.dry * input[i];
            let wet = self.wet * long_lasting[i];
            output.push(dry + wet);
        }

        output
    }
}
