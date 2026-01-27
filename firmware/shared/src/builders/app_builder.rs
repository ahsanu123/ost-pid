use crate::{
    SamplerTrait, builders::app_trait::App, drivers::driver_trait::DriverTrait,
    tasks::task_trait::TaskTrait,
};

pub struct AppBuilderProps<UI, DRIVER, SENSOR>
where
    UI: TaskTrait,
    DRIVER: DriverTrait + TaskTrait,
    SENSOR: SamplerTrait + TaskTrait,
{
    pub ui: UI,
    pub driver: DRIVER,
    pub sensor: SENSOR,
}

pub fn build_app<UI, DRIVER, SENSOR>(
    props: AppBuilderProps<UI, DRIVER, SENSOR>,
) -> App<UI, DRIVER, SENSOR>
where
    UI: TaskTrait,
    DRIVER: DriverTrait + TaskTrait,
    SENSOR: SamplerTrait + TaskTrait,
{
    App {
        ui_task: props.ui,
        driver_task: props.driver,
        sensor_task: props.sensor,
    }
}
