#[cfg(test)]
mod tests {

    use basic_reverb::MonoBasicReverb;
    use std::fs::File;
    use std::path::Path;
    use wav::BitDepth;

    fn get_float_samples(data: BitDepth) -> Vec<f64> {
        let int_samples = data.as_sixteen().expect("Error getting samples").clone();
        let float_samples: Vec<f64> = int_samples
            .into_iter()
            .map(|value| value as f64 / i16::MAX as f64)
            .collect();
        float_samples
    }

    fn get_bit_depth(float_samples: Vec<f64>) -> BitDepth {
        let int_samples: Vec<i16> = float_samples
            .into_iter()
            .map(|value| (value * i16::MAX as f64) as i16)
            .collect();
        BitDepth::from(int_samples)
    }

    #[test]
    fn test_reverb_effect() {
        let mut input_file =
            File::open(Path::new("tests/pulse.wav")).expect("Can't open the input file");
        let (header, data) = wav::read(&mut input_file).expect("Can't convert the input file");

        let input_samples = get_float_samples(data.clone());

        let room_size_ms = 100.;
        let duration_in_seconds = 5.;
        let dry = 0.;
        let wet = 1.;
        const SAMPLE_RATE: u32 = 44100;
        const CHANNELS: usize = 8; // should be a power of 2

        let mut basic_reverb: MonoBasicReverb<CHANNELS, SAMPLE_RATE> =
            MonoBasicReverb::new(room_size_ms, duration_in_seconds, dry, wet);

        let mut output_samples = Vec::with_capacity(input_samples.len());

        for i in 0..input_samples.len() {
            let input_sample = input_samples[i];
            let output_sample = basic_reverb.process_sample(input_sample as f64);
            println!("{}", output_sample);
            output_samples.push(output_sample);
        }

        let data = get_bit_depth(output_samples);
        let mut output_file =
            File::create(Path::new("tests/output.wav")).expect("Can't create the output file");
        wav::write(header, &data, &mut output_file).expect("Can't write the input file");
    }
}
