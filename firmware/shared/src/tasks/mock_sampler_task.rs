use crate::{MockSampler, tasks::task_trait::TaskTrait};
use embassy_futures::select::{Either, select};
use embedded_hal_async::digital::Wait;

impl<IP> TaskTrait for MockSampler<IP>
where
    IP: Wait,
{
    async fn run(&mut self) {
        loop {
            match select(
                self.inc_input.wait_for_falling_edge(),
                self.dec_input.wait_for_falling_edge(),
            )
            .await
            {
                Either::First(_) => self.increase_value(),
                Either::Second(_) => self.decrease_value(),
            }

            self.sampler_sender.send(self.value);
        }
    }
}
