use crate::{SamplerTrait, drivers::driver_trait::DriverTrait, tasks::task_trait::TaskTrait};

pub struct App<UI, DRIVER, SAMPLER>
where
    DRIVER: DriverTrait + TaskTrait,
    SAMPLER: SamplerTrait + TaskTrait,
    UI: TaskTrait,
{
    pub ui_task: UI,
    pub driver_task: DRIVER,
    pub sampler_task: SAMPLER,
}
