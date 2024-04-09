#[cfg(test)]
mod tests {
    use basic_reverb::MonoBasicReverb;
    use std::path::Path;
    // wavers is a separate lib used for the example
    use wavers::{write, Wav};

    #[test]
    fn run_with_file() {
        println!("HERE");
        let input_path: &Path = &Path::new("tests/pulse.wav");
        let output_path: &Path = &Path::new("tests/output.wav");

        let mut wav: Wav<f32> = Wav::from_path(input_path).expect("Input file not found");
        let samples: &mut [f32] = &mut wav.read().expect("Can't decode WAV the file");
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

        println!("{:?}", samples);

        write(output_path, samples, sample_rate, n_channels)
            .expect("Error writing the output file");
    }
}
