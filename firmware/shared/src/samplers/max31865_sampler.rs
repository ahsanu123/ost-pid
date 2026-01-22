use crate::samplers::sampler_trait::SamplerTrait;
use embedded_hal::{delay::DelayNs, spi::SpiBus};
use max31865::Max31865;

pub struct Max31865Sampler<S, D>
where
    S: SpiBus,
    D: DelayNs,
{
    sensor: Max31865<S, D>,
}

impl<S, D> Max31865Sampler<S, D>
where
    S: SpiBus,
    D: DelayNs,
{
    pub fn new(max31865: Max31865<S, D>) -> Self {
        Self { sensor: max31865 }
    }
}

impl<S, D> SamplerTrait for Max31865Sampler<S, D>
where
    S: SpiBus,
    D: DelayNs,
{
    fn sample(&mut self) -> f32 {
        self.sensor.temperature()
    }
}
