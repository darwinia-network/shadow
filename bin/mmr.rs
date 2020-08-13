use mmr::Runner;

fn main() {
    let conn = mmr::store::default_conn();
    let mut runner = Runner::with(&conn);
    runner.start().unwrap();
}
