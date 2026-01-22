use pid::Pid;

const OUT_LIMIT: f32 = 1000.0;
const P_LIMIT: f32 = 1000.0;
const I_LIMIT: f32 = 100.0;
const D_LIMIT: f32 = 100.0;

fn run_controller(pid: &mut Pid<f32>) {
    println!("=========================================");
    for i in 1..15 {
        let sensor_val: f32 = 25.0 * i as f32;

        let out = &pid.next_control_output(sensor_val);
        println!(
            "sensor_val: {:.2},\t out: {:.2},\t p: {:.2},\t i: {},\t d: {:.2}",
            sensor_val, out.output, out.p, out.i, out.d
        );
    }
}

fn main() {
    let mut pid: Pid<f32> = Pid::new(250.0, OUT_LIMIT);
    pid.p(4.45, P_LIMIT);

    run_controller(&mut pid);

    let mut pid: Pid<f32> = Pid::new(250.0, OUT_LIMIT);
    pid.p(4.45, P_LIMIT).i(0.2, I_LIMIT);

    run_controller(&mut pid);

    let mut pid: Pid<f32> = Pid::new(250.0, OUT_LIMIT);
    pid.p(4.45, P_LIMIT).i(0.02, I_LIMIT).d(2.0, D_LIMIT);

    run_controller(&mut pid);
}

