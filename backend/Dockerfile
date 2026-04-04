# 使用官方 Rust 镜像
FROM rust:1.75-slim-bullseye as builder

WORKDIR /app

# 复制后端代码
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# 编译项目
RUN cargo build --release

# 最终镜像
FROM debian:bullseye-slim

# 安装运行时依赖（包括 curl 用于健康检查）
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从 builder 阶段复制编译好的二进制文件
COPY --from=builder /app/target/release/rust-web-crud .

# 暴露端口（PORT 环境变量会在运行时使用）
EXPOSE 3000

# 运行应用 - PORT 环境变量由 Railway 自动设置
ENTRYPOINT ["./rust-web-crud"]