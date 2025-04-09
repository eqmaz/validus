# Suppress command echoing
.SILENT:

# Configuration
PACKAGE_NAME=validus
BUILD_DIR=target
RELEASE_DIR=$(BUILD_DIR)/release
BINARY=$(RELEASE_DIR)/$(PACKAGE_NAME)

# Help
.PHONY: help
help:
	echo "Usage:"
	echo "  make check           - Type check the project"
	echo "  make format          - Format source files"
	echo "  make test            - Run all unit tests"
	echo "  make test-app-core   - Unit tests for app-core framework"
	echo "  make test-trade-core - Unit tests for trade-core library"
	echo "  make build           - Build (dev)"
	echo "  make release         - Build optimized release binary"
	echo "  make release-min     - Build with minimal size optimizations"
	echo "  make run             - Run the project"
	echo "  make start           - Run compiled binary"
	echo "  make clean           - Clean all build artifacts"
	echo "  make update          - Update Cargo dependencies"
	echo "  make all             - Full pipeline: format, check, test, build"

# Basic Commands
.PHONY: check
check:
	cargo check

.PHONY: format
format:
	cargo fmt

.PHONY: test
test:
	cargo test

.PHONY: test-app-core
test-app-core:
	cargo test --package app-core --all-targets --all-features

.PHONY: test-trade-core
test-trade-core:
	cargo test --package trade-core --all-targets --all-features

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: release-min
release-min:
	RUSTFLAGS="-C opt-level=z -C linker-plugin-lto -C strip=symbols" cargo build --release

.PHONY: run
run: format check
	cargo run

.PHONY: start
start:
	$(BINARY)

.PHONY: clean
clean:
	cargo clean

.PHONY: update
update:
	cargo update

.PHONY: all
all: format check test build run
