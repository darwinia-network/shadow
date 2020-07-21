# Shadow

[![Golang CI][workflow-badge]][github]

The shadow service for relayers and verify workers to retrieve header data and generate proof. Shadow will index the data it needs from blockchain nodes, such as Ethereum and Darwinia.

BTW, API docs is [here][api].

## Contribute and Build

> ATTENTION: Please compile shadow under $GOPATH. Here is a guide about [How to setting GOATH](https://github.com/golang/go/wiki/SettingGOPATH).

```
# Clone shadow
git clone https://github.com/darwinia-network/shadow.git

# Make the binary
cd $GOPATH/src/github.com/darwinia-network/shadow && make

# If you are using linux, refresh our static library
sudo ldconfig

# Check the version
./target/shadow version
```

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

## Shadow RPC examples

Exports your `INFURA_KEY` to envrioment

```
export INFURA_KEY='<your-infura-key>'
```

Starts shadow service:

```
# Start shadow service at port 3000
shadow run -v --fetch
```

Avaiable enviroment variables:

| Key              | Description                                                    | default |
|------------------|----------------------------------------------------------------|---------|
| `INFURA_KEY`     | infura key, doesn't know what's [infura][infura]?              | `""`    |
| `SHADOW_GENESIS` | shadow service will block all requests before `SHADOW_GENESIS` | `0`     |
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
[api]: https://darwinia.github.io/shdaow
