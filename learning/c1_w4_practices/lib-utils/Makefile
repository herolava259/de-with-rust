SHELL := /bin/bash
.PHONY: help 

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean:
	cargo clean

build:
	cargo build

lint:
	@rustup component add rustfmt 2> /dev/null
	cargo clippy

fmt:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

test:
	@echo "Running tests..."
	cargo test

make doc:
	@echo "Generating documentation..."
	cargo doc --open

make check:
	@echo "Checking code for errors..."
	cargo check
release:
	@echo "Building release version..."
	cargo build --release
bump:
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"%