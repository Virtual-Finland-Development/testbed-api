builder-image = "virtualfinland/testbed-api-builder"

install:
	docker build --target builder -t ${builder-image} -f infra/builder.dockerfile .
build: install
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} cargo build --release --target-dir /builder/infra/build/target
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} zip -j infra/build/rust.zip ./infra/build/target/release/bootstrap
deploy: build deploy-with-pulumi
deploy-with-pulumi:
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} pulumi -C infra up --yes
clean: install
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} cargo clean --target-dir /builder/infra/build/target
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} rm infra/build/*.zip || true

install-debug:
	docker build --target devenv -t ${builder-image} -f infra/builder.dockerfile .
build-debug: install-debug
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} cargo build --target-dir /builder/infra/build/target
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} zip -j infra/build/rust-debug.zip ./infra/build/target/debug/bootstrap

run: build-debug
	docker run -it --rm -p 3000:3000 \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-v `pwd`:/builder -w /builder \
		virtualfinland/testbed-api-builder \
		sam local start-api --template ./infra/sam-template.yml \
		--host 0.0.0.0 --port 3000
run-native: build-debug
	sam local start-api --template ./infra/sam-template.yml \
		--host 0.0.0.0 --port 3000
