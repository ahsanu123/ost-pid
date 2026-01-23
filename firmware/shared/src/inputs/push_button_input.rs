use crate::inputs::input_trait::{InputTrait, KeyEvent};
use embassy_futures::select::{Either6, select6};
use embedded_hal_async::digital::Wait;

pub struct PushButtonInput<IN>
where
    IN: Wait,
{
    key_up: IN,
    key_right: IN,
    key_down: IN,
    key_left: IN,
    key_back: IN,
    key_enter: IN,
}

impl<IN> PushButtonInput<IN>
where
    IN: Wait,
{
    pub fn new(
        key_up: IN,
        key_right: IN,
        key_down: IN,
        key_left: IN,
        key_back: IN,
        key_enter: IN,
    ) -> Self {
        Self {
            key_up,
            key_right,
            key_down,
            key_left,
            key_back,
            key_enter,
        }
    }
}

impl<IN> InputTrait for PushButtonInput<IN>
where
    IN: Wait,
{
    async fn pool(&mut self) -> KeyEvent {
        let first_event = select6(
            self.key_up.wait_for_falling_edge(),
            self.key_right.wait_for_falling_edge(),
            self.key_down.wait_for_falling_edge(),
            self.key_left.wait_for_falling_edge(),
            self.key_back.wait_for_falling_edge(),
            self.key_enter.wait_for_falling_edge(),
        )
        .await;

        match first_event {
            Either6::First(_) => KeyEvent::Up,
            Either6::Second(_) => KeyEvent::Right,
            Either6::Third(_) => KeyEvent::Down,
            Either6::Fourth(_) => KeyEvent::Left,
            Either6::Fifth(_) => KeyEvent::Back,
            Either6::Sixth(_) => KeyEvent::Enter,
        }
    }
}
