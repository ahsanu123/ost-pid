use crate::{
    samplers::sampler_trait::SamplerTrait,
    singletons::sampler_watcher_singleton::{SAMPLER_WATCHER, SAMPLER_WATCHER_RECEIVER_COUNT},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embedded_hal::{delay::DelayNs, spi::SpiDevice};
use max31865::Max31865;

pub struct Max31865Sampler<S, D>
where
    S: SpiDevice,
    D: DelayNs,
{
    sensor: Max31865<S, D>,
    pub sampler_sender:
        Sender<'static, CriticalSectionRawMutex, f32, SAMPLER_WATCHER_RECEIVER_COUNT>,
}

impl<S, D> Max31865Sampler<S, D>
where
    S: SpiDevice,
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
    S: SpiDevice,
    D: DelayNs,
{
    fn sample(&mut self) -> f32 {
        self.sensor.temperature()
    }
}
