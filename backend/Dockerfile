# 使用官方 Rust 镜像
FROM rust:1.75-slim-bullseye

WORKDIR /app

# 复制后端代码
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# 编译项目
RUN cargo build --release

# 暴露端口
EXPOSE 3000

# 运行应用
CMD ["./target/release/rust-web-crud"]