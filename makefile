EXT := so
SUDO := sudo

ifeq ($(shell uname),Darwin)
    EXT := dylib
	SUDO :=
endif

build: target/release/libmmr.$(EXT)
	@go mod tidy
	@go build -o ./target/shadow -v github.com/darwinia-network/shadow/bin
target/release/libmmr.$(EXT): mmr/lib.rs Cargo.toml
	@cargo build --verbose --release
	$(SUDO) cp ./target/release/libmmr.$(EXT) /usr/local/lib
