use crate::{
    inputs::input_trait::{InputTrait, KeyEvent},
    models::ui_state_model::{Screen, UiState},
    singletons::{
        input_watcher_singleton::INPUT_WATCHER, sampler_watcher_singleton::SAMPLER_WATCHER,
        setpoint_watcher_singleton::SET_POINT_WATCHER,
    },
    tasks::task_trait::TaskTrait,
    ui::display_trait::DisplayTrait,
};
use embassy_futures::select::{Either, select};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::{Receiver, Sender},
};

pub struct UiStateTask<IN, UI>
where
    IN: InputTrait,
    UI: DisplayTrait,
{
    input_watcher: Receiver<'static, CriticalSectionRawMutex, KeyEvent, 2>,
    setpoint_sender: Sender<'static, CriticalSectionRawMutex, f32, 2>,
    sampler_receiver: Receiver<'static, CriticalSectionRawMutex, f32, 4>,

    state: UiState,

    input: IN,
    ui: UI,
}

impl<IN, UI> UiStateTask<IN, UI>
where
    IN: InputTrait,
    UI: DisplayTrait,
{
    pub fn new(input: IN, ui: UI) -> Self {
        let input_watcher = INPUT_WATCHER.receiver().unwrap();
        let setpoint_sender = SET_POINT_WATCHER.sender();
        let sampler_receiver = SAMPLER_WATCHER.receiver().unwrap();

        Self {
            input,
            input_watcher,
            setpoint_sender,
            sampler_receiver,
            ui,
            state: UiState::default(),
        }
    }

    fn handle_dashboard_state_update(&mut self, key: KeyEvent) {
        // TODO:
        match key {
            KeyEvent::Up => todo!(),
            KeyEvent::Right => todo!(),
            KeyEvent::Down => todo!(),
            KeyEvent::Left => todo!(),
            KeyEvent::Back => todo!(),
            KeyEvent::Enter => todo!(),
        }
    }

    fn handle_setting_state_update(&mut self, key: KeyEvent) {
        // TODO:
        match key {
            KeyEvent::Up => todo!(),
            KeyEvent::Right => todo!(),
            KeyEvent::Down => todo!(),
            KeyEvent::Left => todo!(),
            KeyEvent::Back => todo!(),
            KeyEvent::Enter => todo!(),
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        match self.state.screen {
            Screen::Dashboard => self.handle_dashboard_state_update(event),
            Screen::Setting => self.handle_setting_state_update(event),
        };
    }
}

impl<IN, UI> TaskTrait for UiStateTask<IN, UI>
where
    IN: InputTrait,
    UI: DisplayTrait,
{
    async fn run(&mut self) {
        let keyevent = self.input.pool();
        let sampling_val = self.sampler_receiver.changed();

        let come_first = select(keyevent, sampling_val).await;

        match come_first {
            Either::First(keyevent) => {
                self.handle_key_event(keyevent);
                self.ui.update(self.state);
            }

            Either::Second(sampling_val) => {
                self.state.temperature = sampling_val;
                self.ui.update(self.state);
            }
        }
    }
}
