use shadow::Runner;

fn main() {
    env_logger::init();
    let conn = shadow::pool::conn(None);
    let mut runner = Runner::with(conn);
    runner.start().unwrap();
}
