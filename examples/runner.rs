use darwinia_shadow::{db::pool, mmr::Runner};

fn main() {
    env_logger::init();
    let conn = pool::conn(None);
    let mut runner = Runner::with(conn);
    async_std::task::block_on(runner.start()).unwrap();
}
