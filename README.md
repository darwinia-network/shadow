# Shadow

[![CI](https://github.com/darwinia-network/shadow/workflows/CI/badge.svg)](https://github.com/darwinia-network/shadow)
[![crate](https://img.shields.io/crates/v/darwinia-shadow.svg)](https://crates.io/crates/darwinia_shadow)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/darwinia_shadow/)
[![LICENSE](https://img.shields.io/crates/l/darwinia-shadow.svg)](https://choosealicense.com/licenses/gpl-3.0/)

The shadow service for relayers and verify workers to retrieve header data and
generate proof. Shadow will index the data it needs from blockchain nodes, such
as Ethereum and Darwinia.

## Build

```
cargo build --release
```

Note: This will automatically build a `libdarwinia_shadow` dynamic lib
in `/usr/local/lib/`. For static lib link, use

```
LIBRARY_TYPE=static cargo build --release
```

## Usage

```sh
darwinia-shadow 0.7.0

USAGE:
    shadow <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    epoch    Generate epoch data for ethash
    help     Prints this message or the help of the given subcommand(s)
    run      Start shadow service
```

## Download

```sh
$ cargo install darwinia-shadow
```

### Note

+ Please make sure you have `golang` installed in your machine

## Environment Variables

- `ETHEREUM_RPC`

  Optional. The RPC endpoint of an Etherum node, only `http://` and `https://`are
  supported. Default is http://localhost:8545 .

  Example: `http://localhost:8545/`

## Trouble Shooting

Everytime you run `proof` in error, please delete `~/.ethashproof`
and `~/.ethash`
and retry.

## Apis

### Get proofs

1. get the mmr proof of `member` under the root of `last_leaf`'s mountain
2. get the ethash of `target` block

##### REQUEST

`POST /ethereum/ethash_proof`

##### REQUEST PARAMS

```json
{
  "target": 10
  // ethash of target, last_leaf == target - 1
}
```

##### RESPONSE

```json
{
  "ethash_proof": [
    {
      "dag_nodes": [
        "0x5f5a713f8189...",
        "0x0011509c9e55..."
      ],
      "proof": [
        "0x4d1fe9b0c4bd1e33ca4887ed3e49f244",
        ...
      ]
    },
    ...
  ]
}
```

```json
{
  "error": "STRING, error message"
}
```

##### EXAMPLE

```bash
> curl https://shadow.darwinia.network/ethereum/proof \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"target": 10}'
{"ethash_proof":[...]}
```

### Get ethereum tx receipt by tx hash

##### REQUEST

`GET /ethereum/receipt/{tx_hash}`

##### REQUEST PARAMS

`tx_hash`:  ethereum tx hash

##### RESPONSE

```json
{
  "header": {
    "parent_hash": "hash of the parent block",
    "timestamp": INTEGER,
    //the unix timestamp for when the block was collated
    "number": INTEGER,
    // the block number
    "author": "the address of the beneficiary to whom the mining rewards were given",
    "transactions_root": "the root of the transaction trie of the block",
    "uncles_hash": "SHA3 of the uncles data in the block",
    "extra_data": "the extra data field of this block",
    "state_root": "the root of the final state trie of the block",
    "receipts_root": "the root of the receipts trie of the block",
    "log_bloom": "the bloom filter for the logs of the block",
    "gas_used": INTEGER,
    // the total used gas by all transactions in this block
    "gas_limit": INTEGER,
    // the maximum gas allowed in this block
    "difficulty": INTEGER,
    // the difficulty for this block
    "seal": STRING
    ARRAY,
    //
    "hash": "hash of the block"
  },
  "receipt_proof": {
    "index": "0x4b",
    "proof": "",
    "header_hash": ""
  }
}
```

```json
{
  "error": "STRING, error message"
}
```

##### EXAMPLE

```bash
> curl https://shadow.darwinia.network/ethereum/receipt/0x9b8f30bc20809571dd2382433b28d259456cb7f03aec935f6592e1ba1f1173e1
{"header":{...},"receipt_proof":{...}}
```

## LICENSE

GPL-3.0


[github]: https://github.com/darwinia-network/shadow

[workflow-badge]: https://github.com/darwinia-network/shadow/workflows/shadow/badge.svg
