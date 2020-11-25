# Shadow

[![Shadow][workflow-badge]][github]
[![crate](https://img.shields.io/crates/v/darwinia-shadow.svg)](https://crates.io/crates/darwinia_shadow)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/darwinia_shadow/)
[![LICENSE](https://img.shields.io/crates/l/darwinia-shadow.svg)](https://choosealicense.com/licenses/gpl-3.0/)

The shadow service for relayers and verify workers to retrieve header data and generate proof. Shadow will index the data it needs from blockchain nodes, such as Ethereum and Darwinia.


## Usage

```sh
shadow 0.2.5

USAGE:
    shadow <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    count    Current block height in mmr store
    help     Prints this message or the help of the given subcommand(s)
    run      Start shadow service
    trim     Trim mmr from target leaf
```


## Download

```sh
$ cargo install darwinia-shadow
```


### Note

+ Please make sure you have `golang` installed in your machine


## Environment Variables

- `ETHEREUM_RPC`

    Optional. The RPC endpoint of a etherum node, only `http://` and `https://` are supported. Default is http://localhost:8545 .

    Example: `http://localhost:8545/`

- `MMR_LOG`

    Optional. Define how frequently it outputs logs `Pushed mmr ... into database` while generating MMR. Useful when you first time running shadow, since it generates millon of MMR data at first launch. Default is `10000`.

    Example: `"100000"`


## Trouble Shooting

Everytime you run `proof` in error, please delete `~/.ethashproof` and `~/.ethash` 
and retry.


## LICENSE

GPL-3.0


[github]: https://github.com/darwinia-network/shadow
[workflow-badge]: https://github.com/darwinia-network/shadow/workflows/shadow/badge.svg
