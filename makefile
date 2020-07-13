ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

build: target/release/libmmr.$(EXT)
	@go mod tidy
	@go build -o ./target/shadow -v github.com/darwinia-network/shadow/shadow
target/release/libmmr.$(EXT): mmr/lib.rs Cargo.toml
	@cargo build --verbose --release
