use embedded_hal_async::digital::Wait;

use crate::samplers::sampler_trait::SamplerTrait;

pub struct MockSampler<IN>
where
    IN: Wait,
{
    pub value: f32,
    pub inc_input: IN,
    pub dec_input: IN,
}

impl<IN> MockSampler<IN>
where
    IN: Wait,
{
    pub fn increase_value(&mut self) {
        self.value += 1.0;
    }
    pub fn decrease_value(&mut self) {
        self.value -= 1.0;
    }
}

impl<IN> SamplerTrait for MockSampler<IN>
where
    IN: Wait,
{
    fn sample(&mut self) -> f32 {
        self.value
    }
}
