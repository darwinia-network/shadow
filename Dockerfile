# Build Shadow in a stock rust builder container
FROM rust:alpine as shadow
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/Los_Angeles
COPY . shadow

# Required dynamic libraries
#
# libdarwinia_shadow.so => /usr/local/lib/libdarwinia_shadow.so (0x7fd26af02000)
# libssl.so.1.1 => /lib/libssl.so.1.1 (0x7fd26ae81000)
# libcrypto.so.1.1 => /lib/libcrypto.so.1.1 (0x7fd26ac02000)
# libsqlite3.so.0 => /usr/lib/libsqlite3.so.0 (0x7fd26ab1a000)
# libc.musl-x86_64.so.1 => /lib/ld64.so.1 (0x7fd26bebb000)
RUN apk add --no-cache openssl-dev sqlite-dev gcc go libc6-compat musl-dev\
    # && cp /lib/ld-musl-x86_64.so.1 /lib/ld64.so.1 \
    && cd shadow \
    && cargo build --release \
    && mkdir /include \
    && cp target/release/shadow /include/ \
    && cp /lib/libssl.so.1.1 /include/ \
    && cp /lib/libcrypto.so.1.1 /include/ \
    && cp /lib//usr/lib/libsqlite3.so.0 /include/libsqlite3.so.0 \
    && cp /libc.musl-x86_64.so.1 /include/ld64.so.1

# Pull Shadow into a second stage deploy alpine container
FROM alpine:latest
COPY --from=shadow /include /include
RUN mv /include/shadow /usr/local/bin/shadow \
    && mv /include/* /usr/local/lib/
EXPOSE 3000
ENTRYPOINT ["shadow"]
