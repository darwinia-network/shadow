# Shadow

[![Golang CI][workflow-badge]][github]

The shadow service for relayers and verify workers to retrieve header data and generate proof. Shadow will index the data it needs from blockchain nodes, such as Ethereum and Darwinia.

BTW, API docs is [here][api].

## Usage

```sh
$ shadow
The way to Go

Usage:
  shadow [command]

Available Commands:
  epoch       Calculate epoch cache
  header      Get eth block by number
  help        Help about any command
  proof       Proof the block by number
  run         Start shadow service
  version     Print the version number of dargo

Flags:
  -h, --help   help for dargo

Use "shadow [command] --help" for more information about a command.

```

## Getting start with docker

Downloads the docker compose file of shadow

```
$ wget https://raw.githubusercontent.com/darwinia-network/v1/docker-compose/docker-compose.yml
```

Swarm the `docker-compose.yml` file

```
$ docker-compose up
```

This will start:

+ A fetcher service
+ A MMR generating service
+ A geth node


## ## Contribute and Build

Downloads shadow service

```
git clone https://github.com/darwinia-network/shadow.git
```

Builds shadow service

```
cd shadow && make
```

Starts shadow service:

```
# Start shadow service at port 3000
./target/shadow run -v --http 3000
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
