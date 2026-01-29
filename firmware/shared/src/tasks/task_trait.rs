// run should use loop{}, so embassy task
// can directly call task::run() without
// additional loop{} statement
pub trait TaskTrait {
    async fn run(&mut self);
}
