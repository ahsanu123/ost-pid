pub trait DriverTrait {
    fn set_value(&self, value: f32);
}

pub struct SsrDriver {}

impl Default for SsrDriver {
    fn default() -> Self {
        Self {}
    }
}
