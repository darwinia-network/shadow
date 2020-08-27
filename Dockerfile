# Build Shadow in a stock rust builder container
FROM rust:alpine as shadow
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/Los_Angeles
COPY . shadow
RUN apk add --no-cache openssl-dev sqlite-dev gcc go musl-dev\
    && cd shadow \
    && cargo build --release

# Pull Shadow into a second stage deploy alpine container
FROM alpine:latest
COPY --from=shadow /shadow/target/release/shadow /usr/local/bin/shadow
EXPOSE 3000
ENTRYPOINT ["shadow"]
