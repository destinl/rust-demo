# rust-demo

一个简单的全栈Web项目：
- **后端**: Rust + Axum 框架（RESTful API）
- **前端**: React + TypeScript + Vite
- **部署**: Docker + Railway（云部署）

> 这个项目演示了如何在Railway上同时部署前后端服务，实现完整的前后端联通。

## 🚀 快速开始

### 本地开发（Docker Compose）

```bash
# 构建并启动所有服务
docker compose up --build

# 访问应用
# 前端: http://localhost:5173
# 后端API: http://localhost:3000
# 健康检查: http://localhost:3000/health
```

### Railway 云部署

详见 [RAILWAY_DEPLOYMENT.md](./RAILWAY_DEPLOYMENT.md) 完整指南

**核心步骤：**
1. 在 Railway 上分别创建两个服务（后端和前端）
2. 后端服务配置：指向 `backend/Dockerfile`，暴露端口 3000
3. 前端服务配置：指向 `frontend/Dockerfile`，设置环境变量 `BACKEND_URL=<后端的公网URL>`
4. 前端会通过 Nginx 自动代理 `/api/*` 请求到后端

## 📋 API 文档

### 后端 Endpoints

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/health` | 健康检查 |
| `GET` | `/users` | 获取所有用户 |
| `POST` | `/users` | 创建用户 |
| `GET` | `/users/{id}` | 获取单条用户 |
| `PUT` | `/users/{id}` | 修改用户 |
| `DELETE` | `/users/{id}` | 删除用户 |

### 创建用户示例

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"张三","email":"zhangsan@example.com"}'
```

## 📁 项目结构

```
.
├── backend/                    # Rust 后端
│   ├── src/
│   │   └── main.rs            # 主应用程序
│   └── Dockerfile             # 后端容器配置
├── frontend/                   # React 前端
│   ├── src/
│   │   ├── App.tsx            # 主应用组件
│   │   ├── App.css            # 样式
│   │   └── main.tsx
│   ├── index.html
│   ├── Dockerfile             # 前端容器配置
│   └── vite.config.ts
├── docker-compose.yml         # 本地开发配置
├── railway.toml               # Railway 部署配置
└── DEPLOYMENT_GUIDE.md        # 详细部署指南
```

## 🌐 部署方式对比

| 方式 | 适用场景 | 难度 | 成本 |
|------|---------|------|------|
| Docker Compose | 本地开发、测试 | ⭐ 简单 | 免费（本地） |
| Railway | 云上生产部署 | ⭐⭐ 中等 | 免费额度 + 按需计费 |
| Kubernetes | 企业级生产 | ⭐⭐⭐ 复杂 | 高 |

## 🔧 环境变量

### 后端（Rust）

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` | 3000 | 服务监听端口 |
| `RUST_LOG` | info | 日志级别 |

### 前端（Nginx）

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` | 5173 | Nginx监听端口 |
| `BACKEND_URL` | http://rust-backend:3000 | 后端API地址 |

## 🐛 本地开发

### 后端开发

```bash
cd backend
cargo build           # 编译
cargo run            # 运行
cargo test           # 测试
cargo fmt            # 代码格式化
cargo clippy         # 代码检查
```

### 前端开发

```bash
cd frontend
npm install          # 安装依赖
npm run dev         # 开发服务器
npm run build       # 构建
npm run preview     # 预览
```

## 🚦 前后端通信流程

```
用户浏览器
    ↓
前端 (React)
    ↓ (fetch to /api/users)
Nginx (代理)
    ↓ (http://rust-backend:3000/)
后端 (Axum)
    ↓ (返回JSON)
前端 (React)
    ↓
用户浏览器显示数据
```

## 📖 详细指南

- [🚀 部署指南](./DEPLOYMENT_GUIDE.md) - 本地、Railway、生产部署详细步骤
- [🛤️ Railway部署指南](./RAILWAY_DEPLOYMENT.md) - Railway云部署完整教程

## ✅ 关键特性

- ✅ 完整的CRUD操作演示
- ✅ 前后端分离架构
- ✅ Docker容器化部署
- ✅ Railway云部署支持
- ✅ CORS跨域配置
- ✅ Nginx反向代理
- ✅ 健康检查端点
- ✅ 环境变量动态配置

## 🔐 安全性考虑

生产环境建议：
- [ ] 启用HTTPS
- [ ] 配置更严格的CORS策略
- [ ] 添加身份验证 (JWT/OAuth)
- [ ] 实现速率限制
- [ ] 添加输入验证和错误处理
- [ ] 使用数据库而不是内存存储

## 📝 许可证

MIT
