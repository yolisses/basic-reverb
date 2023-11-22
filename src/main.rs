use constants::CHANNELS;

use crate::basic_reverb::BasicReverb;

mod array;
mod basic_reverb;
mod constants;
mod delay;
mod diffuser_half_lengths;
mod diffusion_step;
mod hadmard;
mod hadmard_test;
mod householder;
mod householder_test;
mod multi_channel_mixed_feedback;
mod random_in_range;

pub fn main() {
    let input = [
        0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ];

    let roomSizeMs: f64 = 500.;
    let rt60: f64 = 0.5;
    let dry: f64 = 0.2;
    let wet: f64 = 0.8;
    let mut basic_reverb = BasicReverb::new(roomSizeMs, rt60, dry, wet);

    for sample in input {
        let output = basic_reverb.process([sample; CHANNELS]);
        let mut sum = 0.;
        for c in 0..CHANNELS {
            sum += output[c];
        }
        print!("{}", sum);
    }
}
