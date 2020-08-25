mod mock;

use mock::{ha, header, proof, ETHASH_PROOF_CODEC, ETH_HEADER_THING, HEADER, MOCK_HEADER_19};
use scale::{Decode, Encode};
use shadow::{
    bytes,
    chain::eth::{DoubleNodeWithMerkleProof, EthHeader, HeaderThing},
    hex,
};

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
    assert_eq!(
        format!("08{}", &hex!(&hashes[0..2].concat())),
        hex!(hashes[0..2].to_vec().encode())
    );
}

#[test]
fn eth_header() {
    let header = header();
    let encoded = format!("0x{}", hex!(header.encode()));

    assert_eq!(HEADER, encoded);
    assert_eq!(
        header,
        EthHeader::decode(&mut bytes!(encoded.as_str()).as_ref()).unwrap()
    );
}

#[test]
fn eth_header_thing() {
    assert!(HeaderThing::decode(&mut bytes!(ETH_HEADER_THING).as_ref()).is_ok());
}

#[test]
fn mock_header_thing() {
    HeaderThing::decode(&mut bytes!(MOCK_HEADER_19).as_ref()).unwrap();
}

#[test]
fn decode_mmr_proof() {
    <Vec<[u8; 32]>>::decode(&mut bytes!("0c04bd800035533b44381cb1ef207a3eb00c1e1cee6e561312b44c704a61624dd9e91bc2264d69287157d23d450e34f39925cebe653f3bf02d4a81a8308d02ad9d242553d4c882f9a6c72e34652b5245cc5ff0144037665a30695620ecdbe08c7f").as_ref()).unwrap();
}

#[test]
fn eth_hash_proof() {
    let block =
        <Vec<DoubleNodeWithMerkleProof>>::decode(&mut bytes!(ETHASH_PROOF_CODEC).as_ref()).unwrap();
    assert_eq!(block[block.len() - 1], proof());
}

#[test]
#[cfg(feature = "darwinia")]
fn darwinia_eth_header() {
    use eth::header::EthHeader as DEthHeader;

    let header = header();
    let encoded = format!("0x{}", hex!(header.encode()));

    assert_eq!(HEADER, encoded);
    assert_eq!(
        EthHeader::default().encode(),
        DEthHeader::default().encode()
    );

    assert_eq!(
        header,
        EthHeader::decode(&mut bytes!(encoded.as_str()).as_ref()).unwrap()
    );

    assert_eq!(
        EthHeader::decode(&mut bytes!(encoded.as_str()).as_ref())
            .unwrap()
            .encode(),
        DEthHeader::decode(&mut bytes!(encoded.as_str()).as_ref())
            .unwrap()
            .encode()
    );
}
