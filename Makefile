install:
	cargo install --path .

devinstall:
	cargo install --debug --path .

build:
	cargo build --release -j 16 -v && \
	mv target/release/sipwi .

fmt:
	cargo clean && \
	cargo fmt
