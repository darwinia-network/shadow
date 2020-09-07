FROM ubuntu:latest as builder
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/Los_Angeles
COPY . shadow
RUN apt-get update && apt-get -y upgrade \
    && apt-get -y install golang cargo libssl-dev clang-tools \
    && cd shadow \
    && cargo build --release -vv \
    && mkdir /target \
    && cp /shadow/target/release/shadow /target \
    && cp /usr/local/lib/libdarwinia_shadow.so /target \
    && cp /lib/x86_64-linux-gnu/libssl.so.1.1 /target \
    && cp /lib/x86_64-linux-gnu/libcrypto.so.1.1 /target

FROM ubuntu:latest
COPY --from=builder /target /target
RUN apt-get update -y \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && mv /target/shadow /usr/bin \
    && mv /target/* /usr/lib \
    && rm -rf /target \
    && ldconfig

ENTRYPOINT ["shadow"]

