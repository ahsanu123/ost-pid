use crate::{displays::display_trait::DisplayTrait, models::ui_state_model::UiState};

pub struct ConsoleUi {}

impl ConsoleUi {
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayTrait for ConsoleUi {
    fn update(&mut self, state: UiState) {
        defmt::info!("{}", state)
    }
}
