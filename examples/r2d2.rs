use diesel::{
    prelude::SqliteConnection,
    r2d2::{Builder, ConnectionManager},
};
use std::env;

fn main() {
    let db = env::temp_dir().join("r2d2.db");
    let manager = ConnectionManager::<SqliteConnection>::new(db.to_string_lossy());
    let _ = Builder::new().build(manager).unwrap();
}
