use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

#[derive(Clone, Copy)]
pub struct GlobalStateModel {
    drive_out_val: f32,
    min_out: f32,
    max_out: f32,
}

impl GlobalStateModel {
    pub const fn new() -> Self {
        Self {
            drive_out_val: 10.0,
            min_out: 0.0,
            max_out: 100.0,
        }
    }
}

pub static GLOBAL_STATE: Mutex<CriticalSectionRawMutex, GlobalStateModel> =
    Mutex::new(GlobalStateModel::new());

pub async fn get_state() -> GlobalStateModel {
    let state = GLOBAL_STATE.lock().await;
    *state
}

pub async fn set_state(value: GlobalStateModel) {
    let mut state = GLOBAL_STATE.lock().await;
    *state = value;
}
