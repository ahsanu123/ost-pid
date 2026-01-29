use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

pub const SET_POINT_WATCHER_RECEIVER_COUNT: usize = 2;

pub static SET_POINT_WATCHER: Watch<
    CriticalSectionRawMutex,
    f32,
    SET_POINT_WATCHER_RECEIVER_COUNT,
> = Watch::new();
