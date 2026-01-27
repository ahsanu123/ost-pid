use embedded_hal::digital::OutputPin;

use crate::{SsrDriver, tasks::task_trait::TaskTrait};

impl<OT> TaskTrait for SsrDriver<OT>
where
    OT: OutputPin,
{
    async fn run(&mut self) {
        todo!()
    }
}
