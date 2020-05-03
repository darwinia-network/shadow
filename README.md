<h1 align="center">
𝓭𝓪𝓻𝔀𝓲𝓷𝓲𝓪.𝓰𝓸
</h1>

[![Golang CI][workflow-badge]][github]

## Config

`dargo` use the same config file with `darwinia.js`, if you don't know what 
`darwinia.js` is, run the scripts below before you start

```
mkdir ~/.darwinia
echo '{"eth": { "api": "infura api with your key"}}' > ~/.darwinia/config.json
```

## Installation

Just supports OSX now

```
# Tap darwinia homebrew
brew tap darwinia-network/darwinia

# Install
brew install dargo
```

## Contribute and Build

```
# Clone darwinia.go
git clone https://github.com/darwinia-network/darwinia.go.git

# Make the binary
cd darwinia.go/dargo && make

# Check the version
./target/dargo version
```

## Usage

```sh
$ dargo
The way to Go

Usage:
  dargo [command]

Available Commands:
  epoch       Calculate epoch cache
  header      Get eth block by number
  help        Help about any command
  proof       Proof the block by number
  shadow      Start shadow service
  version     Print the version number of dargo

Flags:
  -h, --help   help for dargo

Use "dargo [command] --help" for more information about a command.

```

## Shadow RPC examples

Fill the `~/.darwinia/config.json`

```
{
  "eth": { 
    "api": "infura-api-with-your-key"
  }
}
```

## Shadow Service

```
# Start shadow service at port 3000
dargo shadow 3000
```

### Enviroment Variables

| Key              | Description                                                    | default |
|------------------|----------------------------------------------------------------|---------|
| `INFURA_KEY`     | infura key, doesn't know what's [infura][infura]?              | `""`    |
| `SHADOW_GENESIS` | shadow service will block all requests before `SHADOW_GENESIS` | `0`     |

The shadow service of dargo follows the [spec][spec].

### Shadow.GetEthHeaderByNumber

```
curl -d '{"method":"shadow_getEthHeaderByNumber","params":{"block_num": 0}, "id": 0}' http://127.0.0.1:3000
```

### Shadow.GetEthHeaderWithProofByNumber

```
curl -d '{"method":"shadow_getEthHeaderWithProofByNumber","params":{"block_num": 1, "transcation": false, "options": {"format": "json"}}, "id": 0}' http://127.0.0.1:3000
```

## Trouble Shooting

Everytime you run `proof` in error, please delete `~/.ethashproof` and `~/.ethash` 
and retry.

## LICENSE

GPL-3.0


[infura]: https://infura.io
[github]: https://github.com/darwinia-network/darwinia.go
[spec]: https://github.com/darwinia-network/darwinia/wiki/Darwinia-offchain-worker-shadow-service-spec
[workflow-badge]: https://github.com/darwinia-network/darwinia.go/workflows/Golang%20CI/badge.svg
