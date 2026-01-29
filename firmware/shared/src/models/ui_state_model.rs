use crate::models::screen_model::Screen;
use defmt::Format;

#[derive(Clone, Copy, PartialEq, PartialOrd, Format)]
pub struct UiState {
    pub setpoint: f32,
    pub temperature: f32,
    pub screen: Screen,
    pub output_val: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            setpoint: 30.0,
            temperature: 25.0,
            screen: Screen::ShowTemperature,
            output_val: 450.0,
        }
    }
}
