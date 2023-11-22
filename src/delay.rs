pub(crate) struct Delay {
    buffer: Vec<f64>,
}

// TODO replace by circular buffer
impl Delay {
    pub(crate) fn reset(&mut self) {
        for element in self.buffer.iter_mut() {
            *element = 0.;
        }
    }

    pub(crate) fn resize(&mut self, min_capacity: i64) {
        // TODO use the previous values instead of hard reset
        self.buffer = vec![0.; min_capacity as usize];
    }

    pub(crate) fn write(&mut self, value: f64) {
        self.buffer.insert(0, value);
        self.buffer.pop();
    }

    pub(crate) fn read(&mut self, delay_samples: i64) -> f64 {
        self.buffer[delay_samples as usize]
    }
}

// new
impl Delay {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.; size],
        }
    }
}
