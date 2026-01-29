use crate::{Max31865Sampler, SamplerTrait, tasks::task_trait::TaskTrait};
use embassy_time::Timer;
use embedded_hal::{
    delay::DelayNs,
    spi::{SpiBus, SpiDevice},
};

impl<S, D> TaskTrait for Max31865Sampler<S, D>
where
    S: SpiDevice,
    D: DelayNs,
{
    async fn run(&mut self) {
        loop {
            let value = self.sample();
            self.sampler_sender.send(value);

            Timer::after_millis(50).await;
        }
    }
}
