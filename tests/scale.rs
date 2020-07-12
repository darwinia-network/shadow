mod mock;

use mmr::{bridge::EthHeader, bytes, hex};
use mock::{ha, header, HEADER};
use scale::{Decode, Encode};

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
