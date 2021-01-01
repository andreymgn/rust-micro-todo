FROM rust:1.49-slim as builder
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/app
COPY . .
ARG service
RUN cargo install --path $service

# -----------------------------------------------------------------------------

FROM debian:buster-slim
ARG service
COPY --from=builder /usr/local/cargo/bin/$service /usr/local/bin/app
CMD ["/usr/local/bin/app"]