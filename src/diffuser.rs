pub(crate) struct DiffuserHalfLengths {
    steps: Vec<DiffusionStep>
}

impl DiffuserHalfLengths {
    pub(crate) fn configure(sampleRate: usize) {
        // for (auto &step : steps) step.configure(sampleRate);
    }


	pub(crate) fn process( samples:&[f64])->[f64] {
		for (auto &step : steps) {
			samples = step.process(samples);
		}
		return samples;
	}
}

