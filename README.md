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
// wavers is a separate lib used for the example
use wavers::{write, Wav};

fn main() {
    let fp: &Path = &Path::new("path/to/the/input.wav");
    let out_fp: &Path = &Path::new("path/to/the/output.wav");

    let mut wav: Wav<f32> = Wav::from_path(fp).unwrap();
    let samples: &mut [f32] = &mut wav.read().unwrap();
    let sample_rate = wav.sample_rate();
    // to simplify, just mono audio
    let n_channels = 1;

    let room_size_ms = 100.;
    let duration_in_seconds = 5.;
    let dry = 0.;
    let wet = 1.;

    // a fixed sample rate is used to improve performance
    const SAMPLE_RATE: u32 = 44100;

    // should be a power of 2
    const CHANNELS: usize = 8;

    let mut basic_reverb: MonoBasicReverb<CHANNELS, SAMPLE_RATE> =
        MonoBasicReverb::new(room_size_ms, duration_in_seconds, dry, wet);

    for i in 0..samples.len() {
        let input_sample = samples[i];
        let output_sample = basic_reverb.process_sample(input_sample as f64);
        samples[i] = output_sample as f32;
    }

    write(out_fp, samples, sample_rate, n_channels).unwrap();
}
```
