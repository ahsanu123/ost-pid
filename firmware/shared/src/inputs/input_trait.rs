pub enum KeyEvent {
    Top,
    Right,
    Down,
    Bottom,
    Back,
    Enter,
}

pub trait InputTrait {
    async fn on_falling_edge<CB>(&mut self, cb: CB)
    where
        CB: Fn();
}

pub struct GpioInput {}

impl Default for GpioInput {
    fn default() -> Self {
        Self {}
    }
}
