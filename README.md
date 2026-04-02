# rust-demo

一个最简单的 Rust Web 项目，使用 `axum` 实现内存级增删改查，并附带 `Dockerfile` 部署方式。

## 运行方式

本地运行：

```bash
cargo run
```

访问：

- `GET /items` - 获取所有项
- `POST /items` - 创建项，JSON body 示例：`{"name":"task"}`
- `GET /items/{id}` - 获取单条项
- `PUT /items/{id}` - 修改项，JSON body 示例：`{"name":"new name"}`
- `DELETE /items/{id}` - 删除项

## 测试

```bash
cargo test
```

## Docker 构建

```bash
docker build -t rust-demo .
docker run -p 3000:3000 rust-demo
```

服务默认监听 `0.0.0.0:3000`。
