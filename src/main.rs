use crate::basic_reverb::BasicReverb;
use constants::CHANNELS;
use hound::{WavReader, WavWriter};
mod array;
mod basic_reverb;
mod constants;
mod delay;
mod diffuser_half_lengths;
mod diffusion_step;
mod mix_matrix;
mod multi_channel_mixed_feedback;

fn main() -> Result<(), hound::Error> {
    // Input WAV file path
    let input_path = "guitar.wav";

    // Output WAV file path
    let output_path = "out.wav";

    // Open the input WAV file for reading
    let mut reader = WavReader::open(input_path).unwrap();

    // Get WAV file metadata
    let spec = reader.spec();

    // Create a new WAV file for writing
    let mut writer = WavWriter::create(output_path, spec)?;

    let room_size_ms: f64 = 100.;
    let rt60: f64 = 10.;
    let dry: f64 = 0.;
    let wet: f64 = 1.;
    let mut basic_reverb = BasicReverb::new(room_size_ms, rt60, dry, wet);

    // run_debug(&mut basic_reverb);

    // Iterate over samples and multiply each sample by 10
    for sample in reader.samples::<i16>() {
        let sample = sample?;
        let sample = sample as f64 / i16::MAX as f64;

        let output_channels = basic_reverb.process([sample; CHANNELS]);
        let mut sum = 0.;

        for c in 0..CHANNELS {
            sum += output_channels[c];
        }

        let processed_sample = (i16::MAX as f64 * sum) as i16;

        // Write the processed sample to the output WAV file
        writer.write_sample(processed_sample)?;
    }

    Ok(())
}
