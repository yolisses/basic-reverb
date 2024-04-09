mod basic_reverb;
mod delay;
mod diffuser;
mod mean;
mod mix_matrix;
mod mono_basic_reverb;
mod multi_channel_mixed_feedback;
mod stereo_basic_reverb;

pub use basic_reverb::BasicReverb;
pub use mono_basic_reverb::MonoBasicReverb;
pub use stereo_basic_reverb::StereoBasicReverb;

pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
