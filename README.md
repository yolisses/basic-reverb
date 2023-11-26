# Basic Reverb

A Rust implementation of the reverb presented at [Let's Write A Reverb](https://youtu.be/6ZK2Goiyotk?si=7DKV5o-jFFr3YSP8). It was made basically translating the [reverb example code](https://github.com/Signalsmith-Audio/reverb-example-code).  
It has differences tough:

- It has a fixed number of channels
- It does not include the examples of intermediary steps to come up with the final reverb.
- It uses a simpler delay (the original it is part of a separate library).
- It doesn't have the `configure` functions so the sample rate can't be changed once set.

## Usage example

```rust
use basic_reverb::MonoBasicReverb;
use std::path::Path;
// wavers is not included on basic-reverb
use wavers::{write, Wav};



fn main() {
    let fp: &Path = &Path::new("path/to/the/input.wav");
    let out_fp: &Path = &Path::new("path/to/the/output.wav");

    let mut wav: Wav<f32> = Wav::from_path(fp).unwrap();
    let input_samples: &mut [f32] = &mut wav.read().unwrap();
    let sample_rate = wav.sample_rate();
    // to simplify, just mono audio
    let n_channels = 1;

    let room_size_ms = 100.;
    let rt60 = 10.;
    let dry = 0.;
    let wet = 1.;

    let mut basic_reverb: MonoBasicReverb<8, 44100> =
        MonoBasicReverb::new(room_size_ms, rt60, dry, wet);

    for i in 0..input_samples.len() {
        let input_sample = input_samples[i];
        let output_sample = basic_reverb.process_sample(input_sample as f64);
        input_samples[i] = output_sample as f32;
    }

    write(out_fp, input_samples, sample_rate, n_channels).unwrap();
}
```
