# 构建阶段
FROM rust:1.75-alpine as builder

# 安装编译依赖
RUN apk add --no-cache musl-dev

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock* ./

# 创建虚拟主程序来缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制源代码并构建
COPY src ./src
RUN touch -a -m ./src/main.rs && \
    cargo build --release

# 运行阶段
FROM alpine:latest

RUN apk add --no-cache ca-certificates

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rust-web-crud /app/rust-web-crud

# 暴露端口
EXPOSE 3000

# 运行应用
CMD ["/app/rust-web-crud"]