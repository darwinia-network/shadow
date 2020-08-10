# Build MMR
FROM rust:latest AS mmr
COPY . .
RUN cargo build --release

# Build Shadow
FROM golang:latest AS shadow
COPY . .
COPY --from=mmr target target
RUN go build -v bin/main.go

# Build Command
FROM scratch
COPY --from=shadow target target
RUN sudo cp .target/release/libmmr.so /usr/local/lib/libmmr.so
CMD ["./target/shadow"]
