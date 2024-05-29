pub trait ProcessorTrait {
    fn compute(&mut self) -> f32;
}

pub struct PidProcessor {}

impl Default for PidProcessor {
    fn default() -> Self {
        Self {}
    }
}
