install:
	cargo install --path .

devinstall:
	cargo install --debug --path .

build:
	cargo build --release -j 16 -v && \
	mv target/release/sipwi .

vscode:
	rm -rf ~/.vscode/extensions/sipwi-lang && \
	cp -r ext ~/.vscode/extensions/sipwi-lang

fmt:
	cargo clean && \
	cargo fmt
