use crate::{displays::display_trait::DisplayTrait, models::ui_state_model::UiState};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget};

pub struct MonochromeLcdDisplay<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    device: D,
}

impl<D> MonochromeLcdDisplay<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    pub fn new(device: D) -> Self {
        Self { device }
    }
}

impl<D> DisplayTrait for MonochromeLcdDisplay<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    fn update(&mut self, state: UiState) {
        todo!()
    }
}
