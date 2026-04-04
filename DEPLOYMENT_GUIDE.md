# 部署指南

## 三种部署方式

### 方式1: Docker Compose (本地开发)

```bash
# 构建并启动所有服务
docker compose up --build

# 访问
# 前端: http://localhost:5173
# 后端: http://localhost:3000
# API健康检查: http://localhost:3000/health
```

### 方式2: Railway (云部署) ⭐ 推荐

详见 [RAILWAY_DEPLOYMENT.md](./RAILWAY_DEPLOYMENT.md)

**快速总结：**
1. 在Railway上分别部署 `backend/` 和 `frontend/`
2. 在前端服务的环境变量中设置 `BACKEND_URL` 指向后端的Public URL
3. 前端会自动通过Nginx代理所有 `/api/*` 请求到后端

**示例：**
```
前端服务环境变量:
BACKEND_URL=https://rust-backend-prod-xxxx.up.railway.app

前端访问流程:
浏览器 → https://react-frontend.up.railway.app/api/users 
         ↓ (Nginx代理)
后端 → https://rust-backend-prod-xxxx.up.railway.app/users
```

### 方式3: Kubernetes (生产环保)

支持使用kubectl部署，配置类似。

---

## 关键改动说明

### ✅ 前端Dockerfile增强
- 支持 `BACKEND_URL` 环境变量动态配置
- Nginx启动时自动生成配置
- 本地开发用 `http://rust-backend:3000`
- Railway部署用后端的Public URL

### ✅ 后端支持
- 已配置CORS (允许所有来源)
- 支持 `PORT` 环境变量
- Railway自动分配PORT环境变量

### ✅ docker-compose.yml优化
- 添加BACKEND_URL环境变量
- 添加健康检查
- 添加服务启动依赖

---

## 本地测试

```bash
# 1. 启动所有服务
docker compose up --build

# 2. 测试前端
curl http://localhost:5173

# 3. 测试后端
curl http://localhost:3000/users

# 4. 在浏览器打开
# http://localhost:5173
# 应该能看到用户列表，并可以添加/编辑/删除用户
```

---

## 前后端通信流程

```
┌─────────────────────────────────────────────────────────┐
│                  用户浏览器                               │
│  访问: https://react-frontend.up.railway.app/           │
└──────────────────────┬──────────────────────────────────┘
                       │
                  (HTTPS请求)
                       │
┌─────────────────────▼──────────────────────────────────┐
│            前端容器 (Nginx Port 5173)                   │
│  ┌─────────────────────────────────────────────────┐   │
│  │ 静态文件: index.html, js, css                   │   │
│  │ /api/* → 代理到后端 (BACKEND_URL env)          │   │
│  └──────────────────────┬──────────────────────────┘   │
└─────────────────────┬───────────────────────────────────┘
                      │
             (根据BACKEND_URL)
                      │
        ┌─────────────┴──────────────┐
        │（本地）                 （Railway）
    http://rust-backend:3000  https://rust-backend.up.railway.app
        │                           │
        └─────────────┬─────────────┘
                      │
    ┌────────────────▼────────────────┐
    │  后端容器 (Axum Port 3000)      │
    │ ┌──────────────────────────────┐│
    │ │ GET /users                   ││
    │ │ POST /users                  ││
    │ │ PUT /users/:id               ││
    │ │ DELETE /users/:id            ││
    │ │ GET /health                  ││
    │ └──────────────────────────────┘│
    └────────────────────────────────┘
```

---

## 环境变量列表

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` (后端) | 3000 | 后端服务监听端口 |
| `PORT` (前端) | 5173 | Nginx监听端口 |
| `BACKEND_URL` (前端) | http://rust-backend:3000 | 后端API地址 |
| `RUST_LOG` (后端) | info | 日志级别 |

---

## 故障排除

### 问题：前端无法连接后端
**检查列表：**
1. ✅ 后端容器是否运行? `docker ps`
2. ✅ 后端是否返回200? `curl http://localhost:3000/health`
3. ✅ Nginx代理配置? `docker logs react-frontend`
4. ✅ BACKEND_URL环境变量是否正确? `docker inspect react-frontend`

### 问题：CORS错误
**原因：** 后端CORS配置或通信方式有问题
**解决：**
- 检查浏览器Network标签的具体错误信息
- 确保API是通过Nginx `/api/` 路由访问

### 问题：Railway部署后前端404
**检查：**
1. 前端Public URL是否可访问
2. `BACKEND_URL` 环境变量是否正确设置
3. 后端Public URL是否可访问

---

## 下一步

- [ ] 测试本地Docker Compose部署
- [ ] 在Railway创建项目
- [ ] 部署后端服务
- [ ] 获取后端Public URL
- [ ] 部署前端服务并配置BACKEND_URL
- [ ] 验证前后端连接
