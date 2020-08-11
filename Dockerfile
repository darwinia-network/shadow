# This Dockerfile only works on linux

FROM rust:alpine

COPY . .

RUN apk add sqlite-dev bash

# FROM golang:1.14-alpine
#
# ARG DEBIAN_FRONTEND=noninteractive
#
# ENV TZ=America/Los_Angeles
#
# COPY target/release/libmmr.so /usr/local/lib
# COPY . shadow
#
# RUN apk add --no-cache sqlite-dev bash musl-dev\
#     && cp /usr/local/lib/libmmr.so /outputs/libmmr.so \
#     && go build -o /outputs/shadow -v /go/shadow/bin/main.go


# ENTRYPOINT ["./shadow"]
