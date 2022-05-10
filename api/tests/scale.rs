use core::num::flt2dec::decode;

use codec::{Decode, Encode};
use mock::{ha, header, proof, ETHASH_PROOF_CODEC, HEADER};
use shadow_types::chain::ethereum::block::EthereumHeader;
use shadow_types::chain::ethereum::ethash::EthashProof;

mod mock;

/// the scale codec of hash is its hex string
#[test]
fn hash() {
    let hashes = ha();
    (0..10).for_each(|i| {
        assert_eq!(hashes[i].encode(), hashes[i]);
    });
}

/// the scale codec of hash array is its concatention
#[test]
fn hash_array() {
    let hashes = ha();
    let encoded = hashes.encode();
    assert_eq!(encoded, hashes.concat());
}

#[test]
fn mmr_proof() {
    let hashes = ha();
    let ha_hex_0 = array_bytes::bytes2hex("08", &hashes[0..2].concat());
    let ha_hex_1 = array_bytes::bytes2hex("08", &hashes[0..2].to_vec().encode());
    assert_eq!(ha_hex_0, ha_hex_1);
}

#[test]
fn eth_header() {
    let header = header();
    let encoded = array_bytes::bytes2hex("0x", header.encode());

    assert_eq!(HEADER, encoded);

    let bytes = array_bytes::hex2bytes(&encoded).unwrap();
    assert_eq!(header, EthereumHeader::decode(&mut bytes.as_ref()).unwrap());
}

#[test]
fn decode_mmr_proof() {
    let bytes = array_bytes::hex2bytes("0c04bd800035533b44381cb1ef207a3eb00c1e1cee6e561312b44c704a61624dd9e91bc2264d69287157d23d450e34f39925cebe653f3bf02d4a81a8308d02ad9d242553d4c882f9a6c72e34652b5245cc5ff0144037665a30695620ecdbe08c7f").unwrap();
    <Vec<[u8; 32]>>::decode(&mut bytes.as_ref()).unwrap();
}

#[test]
fn eth_hash_proof() {
    let bytes = array_bytes::hex2bytes(ETHASH_PROOF_CODEC).unwrap();
    let block = <Vec<EthashProof>>::decode(&mut bytes.as_ref()).unwrap();
    assert_eq!(block[block.len() - 1], proof());
}
