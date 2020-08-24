use mmr::chain::eth::EthHeaderRPCResp;

fn main() {
    let client = reqwest::blocking::Client::new();
    println!("{:?}", EthHeaderRPCResp::get(&client, 1));
}
