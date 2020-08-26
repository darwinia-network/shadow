use shadow::Runner;

fn main() {
    env_logger::init();
    let conn = shadow::pool::conn(None);
    let mut runner = Runner::with(conn);
    async_std::task::block_on(runner.start()).unwrap();
}
