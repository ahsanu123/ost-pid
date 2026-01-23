use crate::models::ui_state_model::UiState;

pub trait DisplayTrait {
    async fn update(&mut self, state: UiState);
}
