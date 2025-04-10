# ----------- Stage 1: Build ----------
FROM rust:1.84 AS builder

# Set up dependencies
WORKDIR /usr/src/app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY src ./src
COPY config ./config
COPY Makefile ./

# Pre-cache dependencies
#RUN cargo build --release || true
# Build statically linked binary using musl so it will run on apline
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy the rest of the project
COPY . .

# Build release binary
RUN make release

# ----------- Stage 2: Runtime ----------
FROM alpine:latest
RUN adduser -D appuser

WORKDIR /app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/validus_trade ./validus
COPY --from=builder /usr/src/app/config ./config

RUN mkdir -p logs && chown appuser:appuser logs

USER appuser

ENTRYPOINT ["./validus"]
