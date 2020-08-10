# This Dockerfile only works on linux

FROM ubuntu:latest

ARG DEBIAN_FRONTEND=noninteractive

ENV TZ=America/Los_Angeles

COPY target/release/libmmr.so /usr/local/lib

COPY target/shadow .

RUN apt-get update -y \
    && apt-get install -y libsqlite3-dev libdbus-1-dev \
    && ldconfig

ENTRYPOINT ["./shadow"]
