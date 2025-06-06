FROM rust:1.83 AS builder

WORKDIR /app

RUN mkdir -p src && echo "fn main() {}" > src/main.rs
COPY Cargo.lock Cargo.toml ./
RUN cargo build --target-dir /usr/local/cargo/bin
COPY . .
RUN touch -a -m src/main.rs
RUN cargo build --target-dir /usr/local/cargo/bin


FROM debian:stable-slim@sha256:dbab92bea4d20d665d158151d5c06fa8d205ab930b344ba949ef323fe98fa663
RUN apt-get update && apt-get install -y lsb-release ca-certificates && apt-get clean all
COPY --from=builder /usr/local/cargo/bin/debug/hello-catalog /usr/local/bin/hello-catalog
ENTRYPOINT ["/usr/local/bin/hello-catalog"]
