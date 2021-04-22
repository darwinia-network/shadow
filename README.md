# Shadow

[![CI](https://github.com/darwinia-network/shadow/workflows/CI/badge.svg)](https://github.com/darwinia-network/shadow)
[![crate](https://img.shields.io/crates/v/darwinia-shadow.svg)](https://crates.io/crates/darwinia_shadow)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/darwinia_shadow/)
[![LICENSE](https://img.shields.io/crates/l/darwinia-shadow.svg)](https://choosealicense.com/licenses/gpl-3.0/)

The shadow service for relayers and verify workers to retrieve header data and generate proof. Shadow will index the data it needs from blockchain nodes, such as Ethereum, huobi-eco-chain, bsc and Darwinia.

## Usage

``` sh
shadow 0.5.0

USAGE:
    shadow <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    count     Current block height in mmr store
    epoch     Generate epoch data for ethereum ethash consensus
    export    Exports shadow's rocksdb
    help      Prints this message or the help of the given subcommand(s)
    import    Imports mmr from shadow backup or geth
    run       Start shadow service
    trim      Trim mmr from target leaf
```

## Download

``` sh
$ cargo install darwinia-shadow
```

### Note

* Please make sure you have `golang` installed in your machine

## Environment Variables

> Suggest to use different environment variables for each shadow instance that connects to various blockchain, but that is not necessary. You **can** indeed use same `local` env to start shadow for different blockchain, however, you **can't** do this with same or different `global` env.

* `ETHEREUM_RPC`

    `Optional` The RPC endpoint of a etherum node, only `http://` and `https://` are supported. Default is http://localhost:8545.

    Example: `http://localhost:8545/`

* `HECO_MAINNET`

    `Optional` You can use this var when you start shadow service for [heco](https://github.com/HuobiGroup/huobi-eco-chain) mainnet mmr generation. The RPC endpoint of a heco mainnet node, only `http://` and `https://` are supported. Default is https://http-mainnet-node.huobichain.com.

* `HECO_TESTNET`

    `Optional` You can use this var when you start shadow service for [heco](https://github.com/HuobiGroup/huobi-eco-chain) testnet mmr generation. The RPC endpoint of a heco testnet node, only `http://` and `https://` are supported. Default is https://http-testnet.huobichain.com.

* `BSC_MAINNET`

    `Optional` You can use this var when you start shadow service for [bsc](https://github.com/binance-chain/bsc) mainnet mmr generation. The RPC endpoint of a bsc mainnet node, only `http://` and `https://` are supported. Default is https://bsc-dataseed.binance.org.

* `BSC_TESTNET`

    `Optional` You can use this var when you start shadow service for [bsc](https://github.com/binance-chain/bsc) testnet mmr generation. The RPC endpoint of a bsc testnet node, only `http://` and `https://` are supported. Default is https://data-seed-prebsc-1-s1.binance.org:8545.

* `MMR_LOG`

    Optional. Define how frequently it outputs logs `Pushed mmr ... into database` while generating MMR. Useful when you first time running shadow, since it generates millon of MMR data at first launch. Default is `10000` .

    Example: `"100000"`

#### Examples
Use same local var to start shadow for different blockchain in same host
```shell
ETHEREUM_RPC=https://ethereum.ethash.rpc.endpoint ./shadow run --mode all --port 3001
ETHEREUM_RPC=https://heco.rpc.endpoint ./shadow run --mode all --port 3002
ETHEREUM_RPC=https://bsc.rpc.endpoint ./shadow run --mode all --port 3003
```

Use different local var to start shadow for different blockchain in same host
``` shell
ETHEREUM_RPC=https://ethereum.ethash.rpc.endpoint ./shadow run --mode all --port 3000
HECO_MAINNET=https://heco.rpc.endpoint ./shadow run --mode all --port 3001
BSC_MAINNET=https://bsc.rpc.endpoint ./shadow run --mode all --port 3002
```

You can left the value empty to activate the default one.
``` shell
HECO_MAINNET= ./shadow run --mode all --port 3003
```


## Sub commands

### import

#### rockdb

If `-u` not set, the default rocksdb dir is ~/.shadow/cache/mmr

example:

``` 

shadow import \
  -p /data/geth/chaindata \
  -u /path/to/rocksdb/dir \ 
  -t 11357653
```

#### mysql

1. create database 'mmr_store'. Any database name can be used.

2. run sub command 'import'

    example:
    

``` bash
    shadow import \
      -p /data/geth/chaindata \
      -u mysql://root:@localhost:3306/mmr_store \
      -t 11357653
```

## Apis

> Cause [heco](https://github.com/HuobiGroup/huobi-eco-chain) and [bsc](https://github.com/binance-chain/bsc) both are forked from etherum, so they share same API route with etherum in shadow service

### Get the total number of leaves

##### REQUEST

 `GET /ethereum/count`

##### RESPONSE

``` json
{
  "error": "INTEGER, the total number of leaves"
}
```

``` json
{ 
  "error": "STRING, error message"
}
```

##### EXAMPLE

``` bash
> curl https://shadow.darwinia.network/ethereum/count
{"count":128}
```

``` bash
> curl https://shadow-heco.darwinia.network/ethereum/count
{"count":128}
```

### Get the mmr leaf by leaf index

##### REQUEST

 `GET /ethereum/mmr_leaf/{leaf_index}`

##### REQUEST PARAMS

`leaf_index` : from 0

##### RESPONSE

``` json
{
  "mmr_leaf": "STRING, the mmr leaf"
}
```

``` json
{ 
  "error": "STRING, error message"
}
```

##### EXAMPLE

``` bash
> curl https://shadow.darwinia.network/ethereum/mmr_leaf/10
{"mmr_leaf":"0x4ff4a38b278ab49f7739d3a4ed4e12714386a9fdf72192f2e8f7da7822f10b4d"}
```

### Get the mmr root by leaf's parent index

##### REQUEST

 `GET /ethereum/parent_mmr_root/{leaf_index}`

##### REQUEST PARAMS

`leaf_index` :  from 0

##### RESPONSE

``` json
{
  "mmr_root": "INTEGER, the mmr root of (leaf_index-1)"
}
```

``` json
{ 
  "error": "STRING, error message"
}
```

##### EXAMPLE

``` bash
> curl https://shadow.darwinia.network/ethereum/parent_mmr_root/10
{"mmr_root":"0xe28d7f650efb9cbaaca7f485d078c0f6b1104807a3a31f85fc1268b0673140ff"}
```

### Get the mmr root by leaf index

##### REQUEST

 `GET /ethereum/mmr_root/{leaf_index}`

##### REQUEST PARAMS

`leaf_index` :  from 0

##### RESPONSE

``` json
{
  "mmr_root": "INTEGER, the mmr root of leaf_index"
}
```

``` json
{ 
  "error": "STRING, error message"
}
```

##### EXAMPLE

``` bash
> curl https://shadow.darwinia.network/ethereum/mmr_root/9
{"mmr_root":"0xe28d7f650efb9cbaaca7f485d078c0f6b1104807a3a31f85fc1268b0673140ff"}
```

### Get proofs

1. get the mmr proof of `member` under the root of `last_leaf`'s mountain
2. get the ethash of `target` block

##### REQUEST

 `POST /ethereum/proof`

##### REQUEST PARAMS

``` json
{
  "member": 2, // leaf index, just to get the mmr proof for this leaf
  "last_leaf": 9, // mmr mountain boundary, mmr_proof_of(member, last_leaf)
  "target": 10 // ethash of target, last_leaf == target - 1
}
```

##### RESPONSE

``` json
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
  ],
  "mmr_proof": [
    "0x3d6122660cc824376f11ee842f83addc3525e2dd6756b9bcf0affa6aa88cf741",
    ...
  ]
}
```

``` json
{ 
  "error": "STRING, error message"
}
```

##### EXAMPLE

``` bash
> curl https://shadow.darwinia.network/ethereum/proof \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"member": 2, "target": 10, "last_leaf": 9}'
{"ethash_proof":[...],"mmr_proof":[...]}
```

### Get ethereum tx receipt by tx hash

##### REQUEST

 `GET /ethereum/receipt/{tx_hash}/{mmr_root_height}`

##### REQUEST PARAMS

`tx_hash` :  ethereum tx hash

`mmr_root_height` : (mmr_root_height - 1) is the mmr leaf index for mountain boundary. 

> mmr_root_height 似乎会产生歧义，建议改掉

##### RESPONSE

``` json
{
  "header": {
    "parent_hash": "hash of the parent block",
    "timestamp": INTEGER, //the unix timestamp for when the block was collated
    "number": INTEGER, // the block number
    "author": "the address of the beneficiary to whom the mining rewards were given",
    "transactions_root": "the root of the transaction trie of the block",
    "uncles_hash": "SHA3 of the uncles data in the block",
    "extra_data": "the extra data field of this block",
    "state_root": "the root of the final state trie of the block",
    "receipts_root": "the root of the receipts trie of the block",
    "log_bloom": "the bloom filter for the logs of the block",
    "gas_used": INTEGER, // the total used gas by all transactions in this block
    "gas_limit": INTEGER, // the maximum gas allowed in this block
    "difficulty": INTEGER, // the difficulty for this block
    "seal": STRING ARRAY, //
    "hash": "hash of the block"
  },
  "receipt_proof": {
    "index": "0x4b",
    "proof": "",
    "header_hash": ""
  },
  "mmr_proof": {
    "member_leaf_index": INTEGER, // just to get the mmr proof for this leaf
    "last_leaf_index": INTEGER, // mmr mountain boundary, mmr_proof_of(member_leaf_index, last_leaf_index)
    "proof": [...]
  }
}
```

``` json
{ 
  "error": "STRING, error message"
}
```

##### EXAMPLE

``` bash
> curl https://shadow.darwinia.network/ethereum/receipt/0x9b8f30bc20809571dd2382433b28d259456cb7f03aec935f6592e1ba1f1173e1/11330897
{"header":{...},"receipt_proof":{...},"mmr_proof":{...}}
```

## LICENSE

GPL-3.0

[github]: https://github.com/darwinia-network/shadow
[workflow-badge]: https://github.com/darwinia-network/shadow/workflows/shadow/badge.svg
