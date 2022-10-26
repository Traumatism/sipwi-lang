build:
	cargo build --release && \
	mv target/release/sipwi . && \
	ls -lh sipwi
