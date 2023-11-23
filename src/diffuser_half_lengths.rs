use crate::{array::Array, constants::CHANNELS, diffusion_step::DiffusionStep};

pub(crate) struct DiffuserHalfLengths {
    steps: Vec<DiffusionStep>,
}

// new
impl DiffuserHalfLengths {
    pub(crate) fn new(mut diffusion_ms: f64) -> Self {
        let mut steps = vec![];

        for _ in 0..CHANNELS {
            let mut step = DiffusionStep::new();
            diffusion_ms *= 0.5;
            step.delay_ms_range = diffusion_ms;
            steps.push(step);
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
