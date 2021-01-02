FROM rust:1.49-slim as planner
WORKDIR app
RUN cargo install cargo-chef --version 0.1.10
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# -----------------------------------------------------------------------------

FROM rust:1.49-slim as cacher
WORKDIR app
RUN cargo install cargo-chef --version 0.1.10
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# -----------------------------------------------------------------------------

FROM rust:1.49-slim as builder
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
WORKDIR app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ARG service
RUN cargo build --release --bin $service

# -----------------------------------------------------------------------------

FROM debian:buster-slim
ARG service
COPY --from=builder /app/target/release/$service /usr/local/bin/app
CMD ["/usr/local/bin/app"]