use crate::{
    DisplayTrait, SamplerTrait, builders::app_trait::App, drivers::driver_trait::DriverTrait,
    inputs::input_trait::InputTrait, tasks::task_trait::TaskTrait,
};

pub struct AppBuilderProps<UI, DRIVER, SENSOR, IN>
where
    UI: TaskTrait,
    DRIVER: DriverTrait + TaskTrait,
    SENSOR: SamplerTrait + TaskTrait,
    IN: InputTrait + TaskTrait,
{
    ui: UI,
    driver: DRIVER,
    sensor: SENSOR,
    input: IN,
}

pub fn build_app<UI, DRIVER, SENSOR, IN>(
    props: AppBuilderProps<UI, DRIVER, SENSOR, IN>,
) -> App<UI, DRIVER, SENSOR, IN>
where
    UI: TaskTrait,
    DRIVER: DriverTrait + TaskTrait,
    SENSOR: SamplerTrait + TaskTrait,
    IN: InputTrait + TaskTrait,
{
    App {
        ui_task: props.ui,
        input_task: props.input,
        driver_task: props.driver,
        sensor_task: props.sensor,
    }
}
