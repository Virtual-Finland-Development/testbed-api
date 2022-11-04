# docker build -t virtualfinland/testbed-api-builder -f infra/builder.dockerfile .

FROM amazonlinux:2 as builder
###
# Install Rust
# @see: https://github.com/ewbankkit/rust-amazonlinux/blob/master/Dockerfile.al2
###

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

FROM builder as devenv

###
# Install cargo extensions
###
RUN cargo install cargo-watch 

###
# Install SAM Cli
###
RUN yum install -y unzip
RUN curl -LO https://github.com/aws/aws-sam-cli/releases/latest/download/aws-sam-cli-linux-x86_64.zip; \
    unzip -q aws-sam-cli-linux-x86_64.zip -d sam-installation; \
    ./sam-installation/install; \
    sam --version; \
    rm -rf aws-sam-cli-linux-x86_64.zip sam-installation

###
# Install pulumi & python for infra
###
ENV PATH="${PATH}:/root/.pulumi/bin"

RUN yum install -y python3 tar gzip; \
    curl -fsSL https://get.pulumi.com | sh; \
    ${HOME}/.pulumi/bin/pulumi version;
COPY ./infra/requirements.txt /tmp/requirements.txt
RUN python3 -m pip install -r /tmp/requirements.txt

###
# Cleanup
###
RUN yum clean all; \
    rm -rf /var/cache/yum; \
    rm -rf /tmp/*;