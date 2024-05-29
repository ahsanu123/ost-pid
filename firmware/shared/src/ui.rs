pub trait UiTrait {
    fn display(&mut self);
}

pub struct ConsoleUi {}

impl Default for ConsoleUi {
    fn default() -> Self {
        Self {}
    }
}
