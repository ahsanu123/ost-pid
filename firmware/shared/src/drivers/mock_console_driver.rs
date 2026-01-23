use crate::drivers::driver_trait::DriverTrait;

#[derive(Default)]
pub struct MockConsoleDriver;

impl DriverTrait for MockConsoleDriver {
    fn set_value(&mut self, value: f32) {
        defmt::info!("out_val: {}", value);
    }
}
