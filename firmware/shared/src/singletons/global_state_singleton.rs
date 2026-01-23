use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{PubSubBehavior, PubSubChannel},
    watch::Watch,
};

const CH_CAP_SIZE: usize = 5;
const CH_SUB_SIZE: usize = 5;
const CH_PUB_SIZE: usize = 4;

const SAMPLER_WATCHER_RECEIVER_COUNT: usize = 4;

pub static GLOBAL_STATE_CH: PubSubChannel<
    CriticalSectionRawMutex,
    GlobalStateMessage,
    CH_CAP_SIZE,
    CH_SUB_SIZE,
    CH_PUB_SIZE,
> = PubSubChannel::new();

pub static SAMPLER_WATCHER: Watch<CriticalSectionRawMutex, f32, SAMPLER_WATCHER_RECEIVER_COUNT> =
    Watch::new();

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    Sampler,
    Input,
}

#[derive(Clone, Copy)]
pub struct GlobalStateMessage {
    pub ty: StateType,
    pub msg: GlobalStateModel,
}

#[derive(Clone, Copy)]
pub struct GlobalStateModel {
    pub output_val: f32,
    pub sampling_val: f32,
}

impl GlobalStateModel {
    pub const fn new() -> Self {
        Self {
            output_val: 30.0,
            sampling_val: 0.0,
        }
    }
}

pub async fn push_global_msg(value: GlobalStateModel, ty: StateType) {
    let msg = GlobalStateMessage { msg: value, ty };

    GLOBAL_STATE_CH.publish_immediate(msg);
}
