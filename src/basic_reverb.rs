use crate::array::Array;
use crate::constants::CHANNELS;
use crate::diffuser_half_lengths::DiffuserHalfLengths;
use crate::multi_channel_mixed_feedback::MultiChannelMixedFeedback;

pub(crate) struct BasicReverb {
    feedback: MultiChannelMixedFeedback,
    diffuser: DiffuserHalfLengths,
    dry: f64,
    wet: f64,
}

impl BasicReverb {
    pub(crate) fn new(room_size_ms: f64, rt60: f64, dry: f64, wet: f64) -> Self {
        let diffuser = DiffuserHalfLengths::new(room_size_ms);
        let mut feedback = MultiChannelMixedFeedback::new();
        feedback.delay_ms = room_size_ms;

        // How long does our signal take to go around the feedback loop?
        let typical_loop_ms = room_size_ms * 1.5;
        // How many times will it do that during our RT60 period?
        let loops_per_rt_60 = rt60 / (typical_loop_ms * 0.001);
        // This tells us how many dB to reduce per loop
        let db_per_cycle = -60. / loops_per_rt_60;

        feedback.decay_gain = f64::powf(10., db_per_cycle * 0.05);

        Self {
            feedback,
            diffuser,
            dry,
            wet,
        }
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        let diffuse = self.diffuser.process(input);
        let long_lasting = self.feedback.process(diffuse);
        let mut output = [0.; CHANNELS];
        for c in 0..CHANNELS {
            output[c] = self.dry * input[c] + self.wet * long_lasting[c];
        }
        output
    }
}
