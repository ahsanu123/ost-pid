use crate::{
    FrrProcessor, SamplerTrait, builders::app_trait::App, drivers::driver_trait::DriverTrait,
    processor::ProcessorTrait, tasks::task_trait::TaskTrait,
};

pub struct AppBuilderProps<UI, DRIVER, SAMPLER>
where
    UI: TaskTrait,
    DRIVER: DriverTrait,
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
    DRIVER: DriverTrait,
    SAMPLER: SamplerTrait + TaskTrait,
{
    let processor = FrrProcessor::new(props.driver);
    App {
        ui_task: props.ui,
        processor_task: processor,
        sampler_task: props.sampler,
    }
}
