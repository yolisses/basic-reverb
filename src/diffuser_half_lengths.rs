use crate::{constants::CHANNELS, diffusion_step::DiffusionStep};

pub(crate) struct DiffuserHalfLengths {
    steps: [DiffusionStep; CHANNELS],
}

// new
impl DiffuserHalfLengths {
    pub(crate) fn new(mut diffusionMs: f64) -> Self {
        let mut steps = [DiffusionStep::new(); CHANNELS];

        for mut step in steps {
            diffusionMs *= 0.5;
            step.delayMsRange = diffusionMs;
        }

        Self { steps }
    }
}
