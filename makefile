ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

build: target/debug/libmmr.$(EXT)
	@go mod tidy
	@go build -o ./target/dargo -v github.com/darwinia-network/darwinia.go/dargo
target/debug/libmmr.$(EXT): mmr/lib.rs Cargo.toml
	@cargo build --verbose --release
