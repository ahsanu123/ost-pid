use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

const SAMPLER_WATCHER_RECEIVER_COUNT: usize = 4;
pub static SAMPLER_WATCHER: Watch<CriticalSectionRawMutex, f32, SAMPLER_WATCHER_RECEIVER_COUNT> =
    Watch::new();
