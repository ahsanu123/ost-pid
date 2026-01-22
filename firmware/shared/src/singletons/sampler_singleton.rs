use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use heapless::Deque;

pub static SAMPLES_BUFFER: Mutex<CriticalSectionRawMutex, Deque<f32, 10>> =
    Mutex::new(Deque::new());

pub async fn push_sample(value: f32) {
    let mut buffer = SAMPLES_BUFFER.lock().await;
    let _ = buffer.push_front(value);
}
