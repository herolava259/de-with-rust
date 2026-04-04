rust-version:
  	@echo "Rust command-line utility versions:"
  	rustc --version
  	cargo --version
  	rustfmt --version
  	rustup --version
  	clippy-driver --version

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet 

test: 
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run 