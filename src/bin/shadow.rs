#[actix_rt::main]
async fn main() {
    darwinia_shadow::cmd::exec().await.unwrap();
}
