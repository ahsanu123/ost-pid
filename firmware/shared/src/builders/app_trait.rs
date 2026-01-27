use crate::{SamplerTrait, drivers::driver_trait::DriverTrait, tasks::task_trait::TaskTrait};

pub struct App<UI, DRIVER, SENSOR>
where
    DRIVER: DriverTrait + TaskTrait,
    SENSOR: SamplerTrait + TaskTrait,
    UI: TaskTrait,
{
    pub ui_task: UI,
    pub driver_task: DRIVER,
    pub sensor_task: SENSOR,
}
