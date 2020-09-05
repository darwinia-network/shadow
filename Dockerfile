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

FROM debian:stretch-slim
#    linux-vdso.so.1 (0x00007fffdafe6000)
#    libstdc++.so.6 => /lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007f5221f0d000)
#    libssl.so.1.1 => /lib/x86_64-linux-gnu/libssl.so.1.1 (0x00007f5221e7a000)
#    libcrypto.so.1.1 => /lib/x86_64-linux-gnu/libcrypto.so.1.1 (0x00007f5221ba4000)
#    libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f5221b9e000)
#    libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f5221b7b000)
#    libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f5221b60000)
#    libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f522196c000)
#    /lib64/ld-linux-x86-64.so.2 (0x00007f5223cb4000)
#    libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f522181d000)
COPY --from=builder /target /target
RUN mv /target/shadow /usr/bin \
    && mv /target/* /usr/local/lib \
    && rm -rf /target \
    && ldconfig

ENTRYPOINT ["shadow"]

