use crate::models::ui_state_model::UiState;

pub trait DisplayTrait {
    fn update(&mut self, state: UiState);
}
