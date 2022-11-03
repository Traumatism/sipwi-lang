build:
	cargo build --release -j 16 -v && \
	mv target/release/sipwi .
