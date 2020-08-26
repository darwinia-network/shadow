# Shadow

[![Golang CI][workflow-badge]][github]

The shadow service for relayers and verify workers to retrieve header data and generate proof. Shadow will index the data it needs from blockchain nodes, such as Ethereum and Darwinia.

## Usage

```sh
shadow 0.1.0

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

## Contribute and Build

Downloads shadow service

```
git clone https://github.com/darwinia-network/shadow.git
```

Starts shadow service:

```
# Starts shadow serives at port 3000
$ cargo run -p 3000

# If you have fast eth node:
$ ETHEREUM_RPC=<your-api> cargo run -p 3000
```

## Trouble Shooting

Everytime you run `proof` in error, please delete `~/.ethashproof` and `~/.ethash` 
and retry.

## LICENSE

GPL-3.0


[infura]: https://infura.io
[github]: https://github.com/darwinia-network/shadow
[spec]: https://github.com/darwinia-network/darwinia/wiki/Darwinia-offchain-worker-shadow-service-spec
[workflow-badge]: https://github.com/darwinia-network/shadow/workflows/Golang%20CI/badge.svg
[api]: https://darwinia-network.github.io/shadow
