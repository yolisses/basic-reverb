use crate::delay::Delay;
use crate::mix_matrix::hadamard::Hadamard;
use array_init::array_init;
use rand::Rng;

pub(crate) struct DiffusionStep<const CHANNELS: usize, const SAMPLE_RATE: u32> {
    pub(crate) delay_ms_range: f64,
    delays: [Delay; CHANNELS],
    flip_polarity: [bool; CHANNELS],
}

impl<const CHANNELS: usize, const SAMPLE_RATE: u32> DiffusionStep<CHANNELS, SAMPLE_RATE> {
    pub(crate) fn new() -> Self {
        let delay_ms_range = 50.;
        let delay_samples_range = delay_ms_range * 0.001 * SAMPLE_RATE as f64;

        let delays = array_init(|i| {
            let range_low = (delay_samples_range * i as f64 / CHANNELS as f64) as i64;
            let range_high = (delay_samples_range * (i as f64 + 1.) / CHANNELS as f64) as i64;

            let mut random = rand::thread_rng();

            let delay_size = random.gen_range(range_low..range_high);
            Delay::new((delay_size + 1) as usize)
        });

        let mut random = rand::thread_rng();
        let flip_polarity = array_init(|_| random.gen_bool(0.5));

        Self {
            delays,
            flip_polarity,
            delay_ms_range,
        }
    }

    pub(crate) fn process(&mut self, input: [f64; CHANNELS]) -> [f64; CHANNELS] {
        // Delay
        let delayed: [f64; CHANNELS] = array_init(|i| {
            self.delays[i].write(input[i]);
            self.delays[i].read()
        });

        // Mix with a Hadamard matrix
        let mut mixed = delayed;
        Hadamard::in_place(&mut mixed);

        // Flip some polarities
        for i in 0..CHANNELS {
            if self.flip_polarity[i] {
                mixed[i] *= -1.;
            }
        }

        mixed
    }
}
