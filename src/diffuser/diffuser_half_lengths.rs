use super::diffusion_step::DiffusionStep;
use array_init::array_init;

pub(crate) struct DiffuserHalfLengths<const CHANNELS: usize, const SAMPLE_RATE: usize> {
    steps: [DiffusionStep<CHANNELS, SAMPLE_RATE>; CHANNELS],
}

impl<const CHANNELS: usize, const SAMPLE_RATE: usize> DiffuserHalfLengths<CHANNELS, SAMPLE_RATE> {
    pub(crate) fn new(diffusion_ms: f64) -> Self {
        let mut diffusion_ms = diffusion_ms;
        let steps = array_init(|_| {
            let mut step = DiffusionStep::new();
            diffusion_ms *= 0.5;
            step.delay_ms_range = diffusion_ms;
            step
        });

        Self { steps }
    }

    pub(crate) fn process(&mut self, mut samples: Vec<f64>) -> Vec<f64> {
        for step in &mut self.steps {
            samples = step.process(samples);
        }
        samples
    }
}
