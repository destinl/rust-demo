FROM rust:1.75-slim-bullseye

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml .

# 复制源代码
COPY src ./src

# 构建应用
RUN cargo build --release

# 暴露端口
EXPOSE 3000

# 运行应用
CMD ["./target/release/rust-web-crud"]
