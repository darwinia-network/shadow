# Shadow

[![Shadow][workflow-badge]][github]

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

## Download

```sh
$ cargo install darwinia-shadow
```

### Note

+ Please make sure you have golang installed in your machine
+ Please make sure you have installed the sqlite3 library in your machine


## Trouble Shooting

Everytime you run `proof` in error, please delete `~/.ethashproof` and `~/.ethash` 
and retry.

## LICENSE

GPL-3.0


[github]: https://github.com/darwinia-network/shadow
[workflow-badge]: https://github.com/darwinia-network/shadow/workflows/shadow/badge.svg
