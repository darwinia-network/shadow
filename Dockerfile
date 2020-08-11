# Build MMR in a stock rust builder container
FROM rust:alpine as mmr
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/Los_Angeles
COPY . shadow
RUN apk add --no-cache sqlite-dev bash musl-dev \
    && cd shadow \
    && cargo build --release


# Build Shadow in a stock Go builder container
FROM golang:1.14-alpine as shadow
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/Los_Angeles
COPY --from=mmr /shadow/target/release/libmmr.a /usr/local/lib/
COPY . shadow
RUN apk add --no-cache sqlite-dev sqlite-libs musl-dev gcc bash \
    && mkdir /outputs \
    && cp /usr/local/lib/libmmr.a /outputs/ \
    && cd shadow \
    && go build -o /outputs/shadow -v bin/main.go


# Pull Geth and Shadow into a third stage deploy alpine container
FROM alpine:latest
COPY --from=shadow /outputs /shadow
COPY --from=ethereum/client-go:latest /usr/local/bin/geth /usr/local/bin/
RUN mv /shadow/libmmr.a /usr/local/lib/ \
    && mv /shadow/shadow /usr/local/bin/ \
    && rm -rf /shadow

ENTRYPOINT ["shadow"]
