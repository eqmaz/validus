# Suppress command echoing
.SILENT:

# Configuration
PACKAGE_NAME=validus_trade
BUILD_DIR=target
RELEASE_DIR=$(BUILD_DIR)/debug
BINARY=$(RELEASE_DIR)/$(PACKAGE_NAME)
DOCKER_IMAGE := validus
DOCKER_CONTAINER := validus

# Help
.PHONY: help
help:
	echo "Usage:"
	echo "  make run             - Compile and run project"
	echo "  make auto            - Run the app, or recompile and run if any source files changed"
	echo "  make start           - Run compiled binary"
	echo "  make all             - Full pipeline: format, check, test, build"
	echo "  make format          - Format source files"
	echo "  make check           - Type check the project"
	echo "  make build           - Build (dev)"
	echo "  make clean           - Clean all build artifacts"
	echo "  make release         - Build optimized release binary"
	echo "  make release-min     - Build with minimal size optimizations"
	echo "  make update          - Update Cargo dependencies"
	echo ""
	echo "REST API Code Generation:"
	echo "  make gen-api         - Generate REST API code"
	echo "  make gen-api-docs    - Generate REST API docs to _docs/api"
	echo ""
	echo "Testing:"
	echo "  make test            - Run all unit tests"
	echo "  make test-app-core   - Unit tests for app-core framework"
	echo "  make test-trade-core - Unit tests for trade-core library"
	echo ""
	echo "Docker Commands:"
	echo "  make docker-build    - Build Docker image for the app"
	echo "  make docker-run      - Run the app container in detached mode"
	echo "  make docker-shell    - Open an interactive shell in the container"
	echo "  make docker-stop     - Stop and remove containers"
	echo "  make docker-clean    - Remove containers, volumes, images"
	echo "  make docker-rebuild  - Clean and rebuild the Docker image"

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

.PHONY: auto
auto: $(BINARY)
	$(BINARY)

$(BINARY): $(shell find src crates -type f -name '*.rs') Cargo.toml Cargo.lock
	cargo build

.PHONY: clean
clean:
	cargo clean

.PHONY: update
update:
	cargo update

.PHONY: all
all: format check test build run

# ==========================
# Code Generation
# ==========================
.PHONY: gen-api
gen-api:
	# Use the java executable directly if the shortcut openapi-generator can't pass on parameters properly
	#java -jar /usr/local/bin/openapi-generator-cli.jar generate \
	openapi-generator generate \
		-g rust-axum \
		-i openapi/rest.yaml \
		-o crates/openapi/

# ==========================
# Docker Commands
# ==========================

# Build the Docker image
docker-build:
	docker compose build

# Run container in detached mode
docker-run:
	docker compose up -d

# Run  container interactively
docker-shell:
	docker run --rm -it \
		-v $$(pwd)/logs:/app/logs \
		--entrypoint /bin/bash \
		$(DOCKER_IMAGE)

# Stop and remove containers
docker-stop:
	docker compose down

# Stop, remove containers & prune images
docker-clean:
	docker compose down --rmi all --volumes --remove-orphans

# Rebuild everything from scratch
docker-rebuild: docker-clean docker-build