FROM ruby:3.2.0-preview1

RUN apt update && apt install -y bison

# Install WASI SDK
RUN curl -L -o wasi-sdk.tar.gz https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-14/wasi-sdk-14.0-linux.tar.gz && \
    tar -xf wasi-sdk.tar.gz

ENV WASI_SDK_PATH="/wasi-sdk-14.0"

# Install Binaryen
RUN curl -L -o binaryen.tar.gz https://github.com/WebAssembly/binaryen/releases/download/version_91/binaryen-version_91-x86_64-linux.tar.gz && \
    tar -xf binaryen.tar.gz

ENV PATH="/binaryen-version_91:${PATH}"

# Clone Ruby Git Repo
RUN git clone https://github.com/ruby/ruby.git

RUN cd ./ruby && \
    ./autogen.sh && \
    ./configure LDFLAGS="-Xlinker -zstack-size=16777216" \
        --host wasm32-unknown-wasi \
        --with-destdir=./ruby-wasm32-wasi \
        --with-static-linked-ext \
        --with-ext=ripper,monitor && \
    make install
