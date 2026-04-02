FROM rust:1.78 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --locked --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/rust_demo /usr/local/bin/
EXPOSE 3000
CMD ["rust_demo"]