use super::diffusion_step::DiffusionStep;

pub(crate) struct DiffuserHalfLengths {
    steps: Vec<DiffusionStep>,
}

impl DiffuserHalfLengths {
    pub(crate) fn new(mut diffusion_ms: f64, channels: usize, sample_rate: u32) -> Self {
        let steps = (0..channels)
            .map(|_| {
                let mut step = DiffusionStep::new(channels, sample_rate);
                diffusion_ms *= 0.5;
                step.delay_ms_range = diffusion_ms;
                step
            })
            .collect();

        Self { steps }
    }

    pub(crate) fn process(&mut self, mut samples: Vec<f64>) -> Vec<f64> {
        for step in &mut self.steps {
            samples = step.process(samples);
        }
        samples
    }
}
