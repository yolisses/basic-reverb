use super::diffusion_step::DiffusionStep;
use crate::array::Array;
use crate::constants::CHANNELS;

pub(crate) struct DiffuserHalfLengths {
    steps: Vec<DiffusionStep>,
}

// new
impl DiffuserHalfLengths {
    pub(crate) fn new(mut diffusion_ms: f64, sample_rate: u32) -> Self {
        let mut steps = vec![];

        for _ in 0..CHANNELS {
            let mut step = DiffusionStep::new(sample_rate);
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
