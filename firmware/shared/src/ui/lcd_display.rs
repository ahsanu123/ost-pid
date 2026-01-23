use crate::{models::ui_state_model::UiState, ui::display_trait::DisplayTrait};

pub struct LcdDisplay {}

impl DisplayTrait for LcdDisplay {
    async fn update(&mut self, state: UiState) {
        todo!()
    }
}
