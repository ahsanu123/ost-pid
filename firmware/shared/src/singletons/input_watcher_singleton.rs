use crate::inputs::input_trait::KeyEvent;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

pub const UI_WATCHER_RECEIVER_COUNT: usize = 2;

pub static INPUT_WATCHER: Watch<CriticalSectionRawMutex, KeyEvent, UI_WATCHER_RECEIVER_COUNT> =
    Watch::new();
