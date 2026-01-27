use crate::{
    FrrProcessor, SamplerTrait, drivers::driver_trait::DriverTrait, tasks::task_trait::TaskTrait,
};

pub struct App<UI, DRIVER, SAMPLER>
where
    DRIVER: DriverTrait,
    SAMPLER: SamplerTrait + TaskTrait,
    UI: TaskTrait,
{
    pub ui_task: UI,
    pub processor_task: FrrProcessor<DRIVER>,
    pub sampler_task: SAMPLER,
}
