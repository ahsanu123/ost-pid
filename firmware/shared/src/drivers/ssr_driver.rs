use crate::drivers::driver_trait::DriverTrait;
use embassy_time::{Duration, Instant};
use embedded_hal::digital::OutputPin;
use num_traits::{clamp, float::FloatCore};

const OUT_LIMIT: f32 = 1000.0;
const P_LIMIT: f32 = 1000.0;
const I_LIMIT: f32 = 100.0;
const D_LIMIT: f32 = 100.0;

pub enum BinaryOutputState {
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
    state: BinaryOutputState,
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
            state: BinaryOutputState::OFF,
        }
    }
}

impl<OT> DriverTrait for SsrDriver<OT>
where
    OT: OutputPin,
{
    fn set_value(&mut self, value: f32) {
        let value = clamp(value, 0.0, OUT_LIMIT);
        // let now = Instant::now();
        // let on_val = Duration::from_millis(value as u64);
        // let off_val = Duration::from_millis((OUT_LIMIT - value) as u64);

        // if (now - self.last_change_state_time) <= (self.state == BinaryOutputState::wwww
        //     match self.state {
        //         BinaryOutputState::ON => {
        //             self.state = BinaryOutputState::OFF;
        //         }
        //         BinaryOutputState::OFF => {
        //             self.state = BinaryOutputState::OFF;
        //         }
        //     }
        // }
        //
        // self.last_output_value = value;
        // self.last_change_state_time = now;
    }
}
