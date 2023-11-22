pub(crate) struct Delay {
    buffer: Vec<f64>,
}

// TODO replace by circular buffer
impl Delay {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.; size],
        }
    }

    pub(crate) fn write(&mut self, value: f64) {
        self.buffer.insert(0, value);
        self.buffer.pop();
    }

    pub(crate) fn read(&self) -> f64 {
        *self
            .buffer
            .last()
            .ok_or("missing buffer last value")
            .unwrap()
    }
}
