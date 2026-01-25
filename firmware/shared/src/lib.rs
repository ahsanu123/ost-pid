#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub(crate) mod builders;
pub(crate) mod constant;
pub(crate) mod drivers;
pub(crate) mod error;
pub(crate) mod inputs;
pub(crate) mod models;
pub(crate) mod processor;
pub(crate) mod samplers;
pub(crate) mod singletons;
pub(crate) mod tasks;
pub(crate) mod ui;

pub use samplers::max31865_sampler::Max31865Sampler;
pub use samplers::mock_sampler::MockSampler;
pub use samplers::sampler_trait::SamplerTrait;

pub use processor::FrrProcessor;

pub use ui::color_lcd_display::ColoredLcdDisplay;
pub use ui::display_trait::DisplayTrait;
pub use ui::monochrome_lcd_dispay::MonochromeLcdDisplay;

pub use models::ui_state_model::Screen;
pub use models::ui_state_model::UiState;

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    #[test]
    fn hello_world() {
        std::println!("hello_world");
    }
}
