use darwinia_shadow::chain::eth::{EthHeader, EthHeaderRPCResp};

fn main() {
    let client = reqwest::Client::new();
    let header = async_std::task::block_on(EthHeaderRPCResp::get(&client, 1))
        .unwrap()
        .result;
    println!("{:?}", &header);
    println!("{:?}", i64::from_str_radix("1f", 16));
    println!("{:?}", Into::<EthHeader>::into(header));
}
