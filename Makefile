builder-image = "virtualfinland/testbed-api-builder"

deploy: build
	python -m pip install -r infra/requirements.txt # TODO: containerize, or handle in CI
	pulumi -C infra up --yes
build: build-builder clean
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} cargo build --release
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} zip -j infra/build/rust.zip ./target/release/bootstrap
clean: build-builder
	docker run -it --rm -v `pwd`:/builder -w /builder ${builder-image} cargo clean
build-builder:
	docker build -t ${builder-image} -f infra/builder.dockerfile .