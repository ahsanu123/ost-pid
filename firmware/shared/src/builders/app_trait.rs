use crate::{
    SamplerTrait, drivers::driver_trait::DriverTrait, inputs::input_trait::InputTrait,
    tasks::task_trait::TaskTrait,
};

pub struct App<UI, DRIVER, SENSOR, IN>
where
    DRIVER: DriverTrait + TaskTrait,
    SENSOR: SamplerTrait + TaskTrait,
    IN: InputTrait + TaskTrait,
    UI: TaskTrait,
{
    pub ui_task: UI,
    pub driver_task: DRIVER,
    pub sensor_task: SENSOR,
    pub input_task: IN,
}
