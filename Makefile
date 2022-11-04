builder-image = "virtualfinland/testbed-api-builder"

install:
	docker build --target builder -t ${builder-image}:builder -f infra/builder.dockerfile .
build: install
	docker run --rm -v `pwd`:/builder -w /builder ${builder-image} cargo build --release --target-dir /builder/infra/build/target
	docker run --rm -v `pwd`:/builder -w /builder ${builder-image} zip -j infra/build/rust.zip ./infra/build/target/release/bootstrap
	docker run --rm -v `pwd`:/builder -w /builder ${builder-image} zip -r infra/build/rust.zip ./openapi
deploy: build deploy-with-pulumi
deploy-with-pulumi:
	pulumi -C infra up --yes

install-dev:
	docker build --target devenv -t ${builder-image}:devenv -f infra/builder.dockerfile .
dev: install-dev
	docker run -it --rm -p 3000:3000 \
		-v `pwd`:/builder -w /builder \
		virtualfinland/testbed-api-builder \
		cargo watch -x 'run --features local-dev'

run-sam: build
	sam local start-api --template ./infra/sam-template.yml \
		--host 0.0.0.0 --port 3000

test: install
	docker run --rm -v `pwd`:/builder -w /builder ${builder-image}:builder cargo test

clean: install
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} cargo clean --target-dir /builder/infra/build/target
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} rm infra/build/*.zip || true