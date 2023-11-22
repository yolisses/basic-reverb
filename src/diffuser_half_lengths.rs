use crate::{constants::CHANNELS, diffusion_step::DiffusionStep};

pub(crate) struct DiffuserHalfLengths {
    steps: [DiffusionStep; CHANNELS],
}

impl DiffuserHalfLengths {
    pub(crate) fn new(diffusionMs: f64) {
        let steps=  [DiffusionStep::ne]
    }
}
