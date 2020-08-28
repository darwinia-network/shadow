#[actix_rt::main]
async fn main() {
    shadow::cmd::exec().await.unwrap();
}
