use crate::{models::ui_state_model::UiState, ui::display_trait::DisplayTrait};

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
