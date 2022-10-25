# @see: https://github.com/ewbankkit/rust-amazonlinux/blob/master/Dockerfile.al2
# docker build -t virtualfinland/testbed-api-builder -f infra/builder.dockerfile .

FROM amazonlinux:2

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.64.0

RUN yum install -y gcc gcc-c++ openssl-devel zip; \
    curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path --profile minimal --default-toolchain $RUST_VERSION -y; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

WORKDIR /builder