install:
	docker run -it --rm -v `pwd`:/app -w /app rust:1.64-slim cargo build --release
	zip -j rust.zip ./target/release/bootstrap