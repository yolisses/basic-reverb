use crate::delay::Delay;
use crate::mix_matrix::hadamard::Hadamard;
use array_init::array_init;
use rand::Rng;

pub(crate) struct DiffusionStep<const CHANNELS: usize, const SAMPLE_RATE: usize> {
    pub(crate) delay_ms_range: f64,
    delays: [Delay; CHANNELS],
    flip_polarity: [bool; CHANNELS],
}

impl<const CHANNELS: usize, const SAMPLE_RATE: usize> DiffusionStep<CHANNELS, SAMPLE_RATE> {
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

    pub(crate) fn process(&mut self, input: Vec<f64>) -> Vec<f64> {
        // Delay
        let delayed: Vec<f64> = self
            .delays
            .iter_mut()
            .enumerate()
            .map(|(index, delay)| {
                delay.write(input[index]);
                delay.read()
            })
            .collect();

        // Mix with a Hadamard matrix
        let mut mixed = delayed;
        Hadamard::in_place(&mut mixed);

        // Flip some polarities
        for (index, flip) in self.flip_polarity.iter().enumerate() {
            if *flip {
                mixed[index] *= -1.;
            }
        }

        mixed
    }
}
