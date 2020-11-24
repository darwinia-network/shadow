use mmr::{
    Runner, Result
};
use primitives::rpc::EthereumRPC;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,mmr");
    }
    env_logger::init();

    let rpcs = vec![
        "https://mainnet.infura.io/v3/b4916cde136d459c8105e497ff300ec7".to_string()
    ];
    let eth = EthereumRPC::new(reqwest::Client::new(), rpcs);
    let runner = Runner::new(eth);
    runner.start().await?;

    Ok(())
}