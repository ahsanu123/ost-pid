// this is driver to handle encoder with additional state machine
// circuit, as explained in this stackoverflow
// https://electronics.stackexchange.com/questions/494616/encoder-acting-as-2-buttons

use embassy_futures::select::{Either3, select3};
use embedded_hal_async::digital::Wait;

use crate::{InputTrait, inputs::input_trait::KeyEvent};

pub struct EncoderAct2ButtonInput<IN>
where
    IN: Wait,
{
    up: IN,
    down: IN,
    pb: IN,
    last_key_ev: Option<KeyEvent>,
}

impl<IN> EncoderAct2ButtonInput<IN>
where
    IN: Wait,
{
    pub fn new(up: IN, down: IN, pb: IN) -> Self {
        Self {
            up,
            down,
            pb,
            last_key_ev: None,
        }
    }
}

impl<IN> InputTrait for EncoderAct2ButtonInput<IN>
where
    IN: Wait,
{
    async fn pool(&mut self) -> KeyEvent {
        let come_first = select3(
            self.pb.wait_for_falling_edge(),
            self.up.wait_for_falling_edge(),
            self.down.wait_for_falling_edge(),
        )
        .await;

        // NOTE:
        // - when user not press pb, it will enter "right" and "left" mode
        // - if user press encoder pb, it will enter "up" and "down" mode
        // - it will return to right and left mode when user press pb again.
        match come_first {
            Either3::First(_) => {
                if self.last_key_ev.is_none() {
                    self.last_key_ev = Some(KeyEvent::Enter);
                    KeyEvent::Enter
                } else {
                    self.last_key_ev = None;
                    KeyEvent::Back
                }
            }
            Either3::Second(_) => {
                if self.last_key_ev.is_none() {
                    KeyEvent::Right
                } else {
                    KeyEvent::Up
                }
            }
            Either3::Third(_) => {
                if self.last_key_ev.is_none() {
                    KeyEvent::Left
                } else {
                    KeyEvent::Down
                }
            }
        }
    }
}
