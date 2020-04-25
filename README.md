<h1 align="center">
𝓭𝓪𝓻𝔀𝓲𝓷𝓲𝓪.𝓰𝓸
</h1>

[![Golang CI][workflow-badge]][github]

## Build 

run `make` under the directory

## Usage

### proof

```shell
./target/proof 0
```

This will return the proof of number 0 block


```shell
./target/proof {"difficulty":"0x400000000","extraData":"0x11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa","gasLimit":"0x1388","gasUsed":"0x0","hash":"0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","miner":"0x0000000000000000000000000000000000000000","mixHash":"0x0000000000000000000000000000000000000000000000000000000000000000","nonce":"0x0000000000000042","number":"0x0","parentHash":"0x0000000000000000000000000000000000000000000000000000000000000000","receiptsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","size":"0x21c","stateRoot":"0xd7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544","timestamp":"0x0","totalDifficulty":"0x400000000","transactions":[],"transactionsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","uncles":[]}
```

You can pass ethereaum header json to `proof` directly, and `proof` will not fetch ethblock itself


### epoch

> NOTE: This binary is optional, just use the `proof` one is fine, but sometimes 
> you want to upgrade the perform of your program, try this.

Calculating the merkle tree for the passing epoch, every 30000 block is an epoch.

`proof` will calculate dag every time the passing block every time they reach an epoch, 
it cost several miniute usually, if you want to `proof` a block in a blazing fast speed, 
run `epoch {epochNumber}` before you `proof {block}`.


#### Proof with epoch
```
# we run `epoch 0` for the 0~30000 blocks
epoch 0

# then, the proof process of `0 ~ 30000` blocks will be blazing fast
proof 0
proof 30000
```

#### Proof without epoch
```
# we just run proof, it will take several minutes because 
# we haven't calcuate merkle dag befaore
proof 0

# this will be blazing fast too, beacause we auto-calcuated
# the merkle tree when we proof 0
proof 30000

# and this time, we must to generate the merkle tree again
proof 30001

# fast
proof 60000
```

## Trouble Shooting

Everytime you run `proof` in error, please delete `~/.ethashproof` and `~/.ethash` 
and retry.

## LICENSE

GPL-3.0


[github]: https://github.com/darwinia-network/darwinia.go
[workflow-badge]: https://github.com/darwinia-network/darwinia.go/workflows/Golang%20CI/badge.svg
