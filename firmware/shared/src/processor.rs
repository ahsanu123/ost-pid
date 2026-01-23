use crate::{
    builders::pid_builder::PidBuilder,
    constant::{D_LIMIT, D_VAL, I_LIMIT, I_VAL, INITIAL_TEMP, OUT_LIMIT, P_LIMIT, P_VAL},
    drivers::driver_trait::DriverTrait,
    singletons::{
        sampler_watcher_singleton::SAMPLER_WATCHER, setpoint_watcher_singleton::SET_POINT_WATCHER,
    },
    tasks::task_trait::TaskTrait,
};
use embassy_futures::select::{Either, select};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Receiver};
use pid::Pid;

pub trait ProcessorTrait {
    async fn compute(&mut self);
}

pub struct FrrProcessor<D>
where
    D: DriverTrait,
{
    pid: Pid<f32>,
    driver: D,
    sampler_receiver: Receiver<'static, CriticalSectionRawMutex, f32, 4>,
    set_point_receiver: Receiver<'static, CriticalSectionRawMutex, f32, 2>,
}

impl<D> FrrProcessor<D>
where
    D: DriverTrait,
{
    pub fn new(driver: D) -> Self {
        let sampler_receiver = SAMPLER_WATCHER.receiver().unwrap();
        let set_point_receiver = SET_POINT_WATCHER.receiver().unwrap();
        let pid = PidBuilder::default();

        Self {
            pid,
            driver,
            sampler_receiver,
            set_point_receiver,
        }
    }
}

impl<D> ProcessorTrait for FrrProcessor<D>
where
    D: DriverTrait,
{
    async fn compute(&mut self) {
        let sensor_val = self.sampler_receiver.changed();
        let setpoint = self.set_point_receiver.changed();

        let come_first = select(sensor_val, setpoint).await;

        match come_first {
            Either::First(sensor_val) => {
                let control = self.pid.next_control_output(sensor_val);
                self.driver.set_value(control.output);
            }
            Either::Second(setpoint) => {
                self.pid.setpoint(setpoint);
            }
        }
    }
}

impl<D> TaskTrait for FrrProcessor<D>
where
    D: DriverTrait,
{
    async fn run(&mut self) {
        self.compute().await;
    }
}
