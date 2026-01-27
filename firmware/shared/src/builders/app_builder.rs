use crate::{
    SamplerTrait, builders::app_trait::App, drivers::driver_trait::DriverTrait,
    tasks::task_trait::TaskTrait,
};

pub struct AppBuilderProps<UI, DRIVER, SAMPLER>
where
    UI: TaskTrait,
    DRIVER: DriverTrait + TaskTrait,
    SAMPLER: SamplerTrait + TaskTrait,
{
    pub ui: UI,
    pub driver: DRIVER,
    pub sampler: SAMPLER,
}

pub fn build_app<UI, DRIVER, SAMPLER>(
    props: AppBuilderProps<UI, DRIVER, SAMPLER>,
) -> App<UI, DRIVER, SAMPLER>
where
    UI: TaskTrait,
    DRIVER: DriverTrait + TaskTrait,
    SAMPLER: SamplerTrait + TaskTrait,
{
    App {
        ui_task: props.ui,
        driver_task: props.driver,
        sampler_task: props.sampler,
    }
}
