build: target/release/libmmr.a
	@go mod tidy
	@go build -o ./target/shadow -v github.com/darwinia-network/shadow/shadow
target/release/libmmr.a: mmr/lib.rs Cargo.toml
	@cargo build --verbose --release
