use crate::{array::Array, constants::CHANNELS, diffusion_step::DiffusionStep};

pub(crate) struct DiffuserHalfLengths {
    steps: Vec<DiffusionStep>,
}

// new
impl DiffuserHalfLengths {
    pub(crate) fn new(mut diffusion_ms: f64) -> Self {
        // Adapt
        let mut steps: Vec<DiffusionStep> = vec![];

        for _ in 0..CHANNELS {
            steps.push(DiffusionStep::new());
        }

        for step in &mut steps {
            diffusion_ms *= 0.5;
            step.delayMsRange = diffusion_ms;
        }

        Self { steps }
    }

    pub(crate) fn process(&mut self, mut samples: Array) -> Array {
        for step in &mut self.steps {
            samples = step.process(samples);
        }
        return samples;
    }
}
