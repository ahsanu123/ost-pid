pub trait SamplerTrait {
    fn sample(&mut self) -> f32;
}

pub struct Max31865Sampler {}

impl Default for Max31865Sampler {
    fn default() -> Self {
        Self {}
    }
}
