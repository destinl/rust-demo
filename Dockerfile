FROM rust:1.76-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --locked --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust_demo /usr/local/bin/rust_demo
EXPOSE 3000
CMD ["rust_demo"]
