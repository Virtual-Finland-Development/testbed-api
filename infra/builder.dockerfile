# docker build -t virtualfinland/testbed-api-builder -f infra/builder.dockerfile .

FROM amazonlinux:2 as builder
###
# Install Rust
# @see: https://github.com/ewbankkit/rust-amazonlinux/blob/master/Dockerfile.al2
###

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.65.0

RUN yum install -y gcc gcc-c++ openssl-devel zip; \
    curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path --profile minimal --default-toolchain $RUST_VERSION -y; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

FROM builder as devenv

###
# Install cargo extensions
###
RUN cargo install cargo-watch 

###
# Cleanup
###
RUN yum clean all; \
    rm -rf /var/cache/yum; \
    rm -rf /tmp/*;