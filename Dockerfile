FROM rust:1 as builder

ENV CARGO_TERM_COLOR=always
ENV LIBRARY_TYPE=static

RUN apt-get update
RUN apt-get install -y clang

##
# Update Rust toolchains
##

ARG RUST_TOOLCHAIN=nightly-2021-02-28
RUN rustup update \
    && rustup install ${RUST_TOOLCHAIN} \
    && rustup default ${RUST_TOOLCHAIN}

##
# Install Go
##

RUN wget https://golang.org/dl/go1.15.1.linux-amd64.tar.gz
RUN tar -C /usr/local -xvzf go*.linux-amd64.tar.gz

ENV PATH="$PATH:/usr/local/go/bin"
RUN go version

##
# Build
##

WORKDIR /src
COPY . .

RUN cargo build --release

##
# Final stage
##

FROM debian:stable-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/target/release/shadow /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/shadow" ]
