use crate::{Max31865Sampler, SamplerTrait, tasks::task_trait::TaskTrait};
use embassy_time::Timer;
use embedded_hal::{delay::DelayNs, spi::SpiBus};

impl<S, D> TaskTrait for Max31865Sampler<S, D>
where
    S: SpiBus,
    D: DelayNs,
{
    async fn run(&mut self) {
        let value = self.sample();
        self.sampler_sender.send(value);

        Timer::after_millis(50).await;
    }
}
