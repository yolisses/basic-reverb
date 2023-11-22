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
    pub(crate) fn new(roomSizeMs: f64, rt60: f64, dry: f64, wet: f64) -> Self {
        let diffuser = DiffuserHalfLengths::new(roomSizeMs);
        let feedback = MultiChannelMixedFeedback::new();
        feedback.delayMs = roomSizeMs;

        // How long does our signal take to go around the feedback loop?
        let typicalLoopMs = roomSizeMs * 1.5;
        // How many times will it do that during our RT60 period?
        let loopsPerRt60 = rt60 / (typicalLoopMs * 0.001);
        // This tells us how many dB to reduce per loop
        let dbPerCycle = -60. / loopsPerRt60;

        feedback.decayGain = f64::powf(10., dbPerCycle * 0.05);

        Self {
            feedback,
            diffuser,
            dry,
            wet,
        }
    }

    pub(crate) fn configure(&mut self, sampleRate: f64) {
        self.feedback.configure(sampleRate);
        self.diffuser.configure(sampleRate);
    }

    pub(crate) fn process(&mut self, input: Array) -> Array {
        let diffuse = self.diffuser.process(input);
        let longLasting = self.feedback.process(diffuse);
        let output = [0.; CHANNELS];
        for c in 0..CHANNELS {
            output[c] = self.dry * input[c] + self.wet * longLasting[c];
        }
        output
    }
}
