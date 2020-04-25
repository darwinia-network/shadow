build:
	@rm -rf target
	@go mod tidy
	@go build -o ./target/proof -v github.com/darwinia-network/darwinia.go/cmd/proof
	@go build -o ./target/epoch -v github.com/darwinia-network/darwinia.go/cmd/epoch
build-all: deps
	@rm -rf ./target/all
	@mkdir -p ./target/all && cd ./target/all
	@xgo --targets=linux,darwin/amd64 github.com/darwinia-network/darwinia.go/cmd/proof
	@xgo --targets=linux,darwin/amd64 github.com/darwinia-network/darwinia.go/cmd/epoch
deps:
	@docker pull karalabe/xgo-latest
	@go get github.com/karalabe/xgo
tar:
	@rm -rf ./target/tars
	@tar -czvf ./target/tars/darwinia.go-darwin.tar.gz ./target/*darwin*
	@tar -czvf ./target/tars/darwinia.go-linux.tar.gz ./target/*linux-amd64*
all: build-all tar
