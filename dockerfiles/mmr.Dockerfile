# Build MMR in a stock rust builder container
FROM rust:alpine as mmr-builder
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/Los_Angeles
COPY . /shadow
RUN apk add --no-cache sqlite-dev bash musl-dev \
     && cd /shadow \
     && cargo build --release

# Pull mmr into a second stage deploy alpine container
FROM alpine:latest
COPY --from=mmr-builder /shadow/target/release/mmr /usr/local/bin/
ENTRYPOINT ["mmr"]
