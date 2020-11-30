#[actix_rt::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    darwinia_shadow::cmd::exec().await.unwrap();
}
