builder-image = "virtualfinland/testbed-api-builder"

install:
	docker build --target builder -t ${builder-image} -f infra/builder.dockerfile .
build: install
	docker run --rm -v `pwd`:/builder -w /builder -e CARGO_HOME=/builder/.cargo_home ${builder-image} cargo build --release --target-dir /builder/infra/build/target
	docker run --rm -v `pwd`:/builder -w /builder -e CARGO_HOME=/builder/.cargo_home ${builder-image} zip -j infra/build/rust.zip ./infra/build/target/release/bootstrap
	docker run --rm -v `pwd`:/builder -w /builder -e CARGO_HOME=/builder/.cargo_home ${builder-image} zip -r infra/build/rust.zip ./openapi
	
deploy: build deploy-with-pulumi
deploy-with-pulumi:
	pulumi -C infra up --yes

install-dev:
	docker build --target devenv -t ${builder-image}:devenv -f infra/builder.dockerfile .
dev: install-dev
	docker run -it --rm -p 3003:3000 \
		-v `pwd`:/builder -w /builder \
		-e LOGGING_LEVEL=debug \
		-e STAGE=local \
		-e CARGO_HOME=/builder/.cargo_home \
		virtualfinland/testbed-api-builder:devenv \
		cargo watch -x 'run --features local-dev'

run:
	LOGGING_LEVEL=debug STAGE=local cargo watch -x 'run --features local-dev'
run-sam: build
	sam local start-api --template ./infra/sam-template.yml \
		--host 0.0.0.0 --port 3003

test: install
	docker run --rm -v `pwd`:/builder -w /builder -e CARGO_HOME=/builder/.cargo_home ${builder-image} cargo test
	docker run --rm -v `pwd`:/builder -w /builder -e CARGO_HOME=/builder/.cargo_home ${builder-image} cargo test -p api_app

clean: install
	docker run -it --rm -v `pwd`:/builder -w /builder -e CARGO_HOME=/builder/.cargo_home ${builder-image} cargo clean --target-dir /builder/infra/build/target
	docker run -it --rm -v `pwd`:/builder -w /builder -e CARGO_HOME=/builder/.cargo_home ${builder-image} rm infra/build/*.zip || true