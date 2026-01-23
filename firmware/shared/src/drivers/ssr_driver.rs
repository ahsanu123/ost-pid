use crate::{constant::OUT_LIMIT, drivers::driver_trait::DriverTrait};
use embassy_time::{Duration, Instant};
use embedded_hal::digital::OutputPin;
use num_traits::{clamp, float::FloatCore as _};

#[derive(PartialEq, Eq)]
pub enum OutputState {
    ON,
    OFF,
}

pub struct SsrDriver<OT>
where
    OT: OutputPin,
{
    pin: OT,
    last_change_state_time: Instant,
    last_output_value: f32,
    state: OutputState,
}

impl<OT> SsrDriver<OT>
where
    OT: OutputPin,
{
    pub fn new(pin: OT) -> Self {
        Self {
            pin,
            last_output_value: 0.0,
            last_change_state_time: Instant::now(),
            state: OutputState::OFF,
        }
    }
}

impl<OT> DriverTrait for SsrDriver<OT>
where
    OT: OutputPin,
{
    fn set_value(&mut self, value: f32) {
        let on_clamped_value = clamp(value, 0.0, OUT_LIMIT).round() as u64;
        let off_clamped_value = clamp(OUT_LIMIT - value, 0.0, OUT_LIMIT).round() as u64;

        let now = Instant::now();
        let on_val = Duration::from_millis(on_clamped_value);
        let off_val = Duration::from_millis(off_clamped_value);

        let delay = if self.state == OutputState::ON {
            on_val
        } else {
            off_val
        };

        if (now - self.last_change_state_time) > delay {
            match self.state {
                OutputState::ON => {
                    let _ = self.pin.set_low();
                    self.state = OutputState::OFF;

                    self.last_output_value = value;
                    self.last_change_state_time = now;
                }
                OutputState::OFF => {
                    let _ = self.pin.set_high();
                    self.state = OutputState::ON;

                    self.last_output_value = value;
                    self.last_change_state_time = now;
                }
            }
        }
    }
}
