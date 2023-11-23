pub(crate) struct Delay {
    index: usize,
    buffer: Vec<f64>,
}

impl Delay {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            index: 0,
            buffer: vec![0.; size],
        }
    }

    pub(crate) fn write(&mut self, value: f64) {
        self.buffer[self.index] = value;
        self.index += 1;
        if self.index >= self.buffer.len() {
            self.index = 0;
        }
    }

    pub(crate) fn read(&self) -> f64 {
        self.buffer[self.index]
    }
}
