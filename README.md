# Testbed API

A backend service for frontend client calls to testbed

## Usage

### Requirements

- SAM cli: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html
- Docker: https://docs.docker.com/get-docker/
- make: https://www.gnu.org/software/make/

### Run locally

```bash
make run-native
```

The server should respond to http://localhost:3000

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

### Docker

- https://hub.docker.com/_/rust/
