use crate::{
    samplers::sampler_trait::SamplerTrait, singletons::global_state_singleton::SAMPLER_WATCHER,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embedded_hal::{delay::DelayNs, spi::SpiBus};
use max31865::Max31865;

pub struct Max31865Sampler<S, D>
where
    S: SpiBus,
    D: DelayNs,
{
    sensor: Max31865<S, D>,
    pub sampler_sender: Sender<'static, CriticalSectionRawMutex, f32, 4>,
}

impl<S, D> Max31865Sampler<S, D>
where
    S: SpiBus,
    D: DelayNs,
{
    pub fn new(max31865: Max31865<S, D>) -> Self {
        let sampler_sender = SAMPLER_WATCHER.sender();
        Self {
            sensor: max31865,
            sampler_sender,
        }
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
