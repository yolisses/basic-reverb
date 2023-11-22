pub(crate) struct BasicReverb {}

impl BasicReverb {
    pub(crate) fn new(roomSizeMs: f64, rt60: f64, dry: f64, wet: f64) -> Self {
        // feedback.delayMs = roomSizeMs;

        // How long does our signal take to go around the feedback loop?
        let typicalLoopMs = roomSizeMs * 1.5;
        // How many times will it do that during our RT60 period?
        let loopsPerRt60 = rt60 / (typicalLoopMs * 0.001);
        // This tells us how many dB to reduce per loop
        let dbPerCycle = -60. / loopsPerRt60;

        // feedback.decayGain = std::pow(10, dbPerCycle*0.05);
    }

    pub(crate) fn configure(sampleRate: usize) {
        // feedback.configure(sampleRate);
        // diffuser.configure(sampleRate);
    }


pub(crate)	 fn process( input:&[f64]) {
		let diffuse = diffuser.process(input);
		let longLasting = feedback.process(diffuse);
		let output: ();
		for (int c = 0; c < channels; ++c) {
			output[c] = dry*input[c] + wet*longLasting[c];
		}
		return output;
	}
}
