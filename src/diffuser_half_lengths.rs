use crate::{array::Array, constants::CHANNELS, diffusion_step::DiffusionStep};

pub(crate) struct DiffuserHalfLengths {
    steps: Vec<DiffusionStep>,
}

// new
impl DiffuserHalfLengths {
    pub(crate) fn new(mut diffusionMs: f64) -> Self {
        // Adapt
        let mut steps: Vec<DiffusionStep> = vec![];

        for _ in 0..CHANNELS {
            steps.push(DiffusionStep::new());
        }

        for step in &mut steps {
            diffusionMs *= 0.5;
            step.delayMsRange = diffusionMs;
        }

        Self { steps }
    }

    pub(crate) fn configure(&mut self, sampleRate: f64) {
        for step in &mut self.steps {
            step.configure(sampleRate);
        }
    }

    pub(crate) fn process(&mut self, mut samples: Array) -> Array {
        for step in &mut self.steps {
            samples = step.process(samples);
        }
        return samples;
    }
}
