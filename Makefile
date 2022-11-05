install:
	cargo install --path .

build:
	cargo build --release -j 16 -v && \
	mv target/release/sipwi .

fmt:
	cargo clean && \
	cargo fmt
