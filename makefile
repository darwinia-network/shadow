EXT := so
SUDO := sudo

ifeq ($(shell uname),Darwin)
    EXT := dylib
	SUDO :=
endif

# cp target/release/libmmr.$(EXT) /usr/local/lib

build: target/release/libmmr.$(EXT)
	@go mod tidy
	@go build -o ./target/shadow -v github.com/darwinia-network/shadow/shadow
target/release/libmmr.$(EXT): mmr/lib.rs Cargo.toml
	@cargo build --verbose --release
	$(SUDO) cp ./target/release/libmmr.$(EXT) /usr/local/lib
