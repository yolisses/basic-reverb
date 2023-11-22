use crate::basic_reverb::BasicReverb;
use constants::CHANNELS;
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
mod sample_rate;

pub fn main() {
    let input = [
        0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ];

    let room_size_ms: f64 = 500.;
    let rt60: f64 = 0.5;
    let dry: f64 = 0.2;
    let wet: f64 = 0.8;
    let mut basic_reverb = BasicReverb::new(room_size_ms, rt60, dry, wet);

    let mut result = vec![];
    for sample in input {
        let output = basic_reverb.process([sample; CHANNELS]);
        let mut sum = 0.;
        for c in 0..CHANNELS {
            sum += output[c];
        }
        result.push(sum);
    }

    for value in result {
        print!("{:.2e} ", value);
    }
}
