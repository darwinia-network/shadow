use mmr::Runner;

fn main() {
    env_logger::init();
    let conn = mmr::pool::conn(None);
    let mut runner = Runner::with(conn);
    runner.start().unwrap();
}
