use shadow::chain::eth::{EthHeader, EthHeaderRPCResp};

fn main() {
    let client = reqwest::blocking::Client::new();
    let header = EthHeaderRPCResp::get(&client, 1).unwrap().result;
    println!("{:?}", &header);
    println!("{:?}", i64::from_str_radix("1f", 16));
    println!("{:?}", Into::<EthHeader>::into(header));
}
