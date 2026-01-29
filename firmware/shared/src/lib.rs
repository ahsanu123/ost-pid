#![no_std]

#[cfg(feature = "std")]
extern crate std;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::pixelcolor::Rgb888;

pub(crate) mod builders;
pub(crate) mod constant;
pub(crate) mod displays;
pub(crate) mod drivers;
pub(crate) mod error;
pub(crate) mod inputs;
pub(crate) mod models;
pub(crate) mod processor;
pub(crate) mod samplers;
pub(crate) mod singletons;
pub(crate) mod tasks;

pub use tasks::task_trait::TaskTrait;
pub use tasks::ui_state_task::UiStateTask;

pub use samplers::max31865_sampler::Max31865Sampler;
pub use samplers::mock_sampler::MockSampler;
pub use samplers::sampler_trait::SamplerTrait;

pub use processor::FrrProcessor;
pub use processor::ProcessorTrait;

pub use inputs::input_trait::InputTrait;
pub use inputs::push_button_input::PushButtonInput;

pub use displays::color_lcd_display::ColoredLcdDisplay;
pub use displays::display_trait::DisplayTrait;
pub use displays::monochrome_lcd_dispay::MonochromeLcdDisplay;

pub use models::screen_model::Screen;
pub use models::ui_state_model::UiState;

pub use builders::app_builder::AppBuilderProps;
pub use builders::app_builder::build_app;
pub use builders::app_trait::App;
pub use builders::pid_builder::PidBuilder;

pub use drivers::driver_trait::DriverTrait;
pub use drivers::ssr_driver::OutputState;
pub use drivers::ssr_driver::SsrDriver;
pub use max31865::Max31865;

pub mod prelude {
    pub use super::*;
}

// pub fn graphic_conn<T>(display: T)
// where
//     T: embedded_graphics::prelude::DrawTarget<Color = Rgb888>,
// {
//     todo!()
// }

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    #[test]
    fn hello_world() {
        std::println!("hello_world");
    }
}
