// TODO consider removing this derive
#[derive(Clone, Copy)]
pub(crate) struct Delay {}

impl Delay {
    pub(crate) fn reset(&mut self) {
        // TODO
    }

    pub(crate) fn resize(&mut self, minCapacity: i64) {
        // TODO
    }

    pub(crate) fn write(&mut self, value: f64) {
        // TODO
    }

    pub(crate) fn read(&mut self, delaySamples: i64) -> f64 {
        // TODO
        0.
    }
}

// new
impl Delay {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
