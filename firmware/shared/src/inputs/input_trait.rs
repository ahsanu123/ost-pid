#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyEvent {
    Up,
    Right,
    Down,
    Left,
    Back,
    Enter,
}

pub trait InputTrait {
    async fn pool(&mut self) -> KeyEvent;
}
