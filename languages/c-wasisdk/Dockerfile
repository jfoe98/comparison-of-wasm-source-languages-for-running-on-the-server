FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y wget && \
    apt-get install -y libxml2

ENV WASI_VERSION=16
ENV WASI_VERSION_FULL=${WASI_VERSION}.0
RUN mkdir /wasi-sdk && \
    cd /wasi-sdk && \
    wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
    tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz && \
    cp -r /wasi-sdk/wasi-sdk-${WASI_VERSION_FULL} /opt/wasi-sdk

ENV WASI_SDK_PATH=/opt/wasi-sdk

WORKDIR /build
