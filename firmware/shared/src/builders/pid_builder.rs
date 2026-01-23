use crate::constant::{D_LIMIT, D_VAL, I_LIMIT, I_VAL, INITIAL_TEMP, OUT_LIMIT, P_LIMIT, P_VAL};
use pid::Pid;

pub struct PidBuilder {}

impl PidBuilder {
    pub fn default() -> Pid<f32> {
        let mut pid: Pid<f32> = Pid::new(INITIAL_TEMP, OUT_LIMIT);
        pid.p(P_VAL, P_LIMIT).i(I_VAL, I_LIMIT).d(D_VAL, D_LIMIT);

        pid
    }
}
