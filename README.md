# Testbed API

A backend service for frontend client calls to testbed

[![Deploy Dev](https://github.com/Virtual-Finland-Development/testbed-api/actions/workflows/deploy-dev.yml/badge.svg?branch=main)](https://github.com/Virtual-Finland-Development/testbed-api/actions/workflows/deploy-dev.yml)

[![Deploy Staging](https://github.com/Virtual-Finland-Development/testbed-api/actions/workflows/deploy-staging.yml/badge.svg?branch=main)](https://github.com/Virtual-Finland-Development/testbed-api/actions/workflows/deploy-staging.yml)

## Usage

### Requirements

- Docker: https://docs.docker.com/get-docker/

### Run with docker with hot reloading

with docker-compose:

```bash
docker compose up
```

After a log message: ` Running 'target/debug/bootstrap'`, the server should respond to http://localhost:3003 with a redirect to the swagger documentation page

### Natively with rust

Requires rust and cargo watch to be installed

- rust: https://www.rust-lang.org/tools/install
- cargo watch: https://github.com/watchexec/cargo-watch

```base
cargo watch -x 'run --features local-dev'
```

### Simulate the lambda runtime with a local SAM Client

Requires a local installation of SAM Client:

- SAM cli: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html
- make: https://www.gnu.org/software/make/

```bash
make run-sam
```

## References

### Rust AWS Lambda REST API starting points

- https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
- https://hevodata.com/learn/rust-lambda/
- https://awslabs.github.io/aws-lambda-rust-runtime/lambda_http/index.html

### Libraries / Crates

- https://crates.io/crates/lambda_runtime
- https://crates.io/crates/serde
- https://crates.io/crates/tokio
- https://crates.io/crates/reqwest

### Hot reloading

- https://github.com/watchexec/cargo-watch
- https://github.com/rksm/rust-hot-reload

### Docker

- https://hub.docker.com/_/rust/
