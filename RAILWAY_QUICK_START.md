# Railway 部署快速参考卡

## 🎯 一句话总结
在Railway上分别部署前后端Dockerfile，通过环境变量 `BACKEND_URL` 实现Nginx动态代理，完成前后端联通。

## ⚡ 3步部署

### Step 1: 登录 Railway
- 访问 https://railway.app
- GitHub 账户登录或注册

### Step 2: 创建两个服务

**后端服务：**
```
Name:              rust-backend
Dockerfile Path:   backend/Dockerfile
Port:              3000
Environment:       (默认即可)
```

**前端服务：**
```
Name:              react-frontend
Dockerfile Path:   frontend/Dockerfile
Port:              5173
Environment:       BACKEND_URL = <后端的Public URL>
```

### Step 3: 获取前端 Public URL，访问应用

```
https://react-frontend-production-xxxx.up.railway.app
```

---

## 🔑 关键配置项

### 前端 (Dockerfile 自动化处理)
```dockerfile
ENV BACKEND_URL=http://rust-backend:3000  # 默认值
ENV PORT=5173

# 启动脚本自动使用这些环境变量生成 Nginx 配置
/docker-entrypoint.sh → 生成 /etc/nginx/conf.d/default.conf
```

### 后端 (已支持)
```rust
// 支持环境变量 PORT
let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string())

// CORS 已配置
CorsLayer::new().allow_origin(Any)
```

---

## 🌐 通信方式

### 本地开发 (docker-compose)
```
浏览器 → http://localhost:5173
         ↓ (Nginx代理 /api/*)
后端 ← http://rust-backend:3000
```

### Railway 生产
```
浏览器 → https://react-frontend.up.railway.app
         ↓ (Nginx代理 /api/*)
后端 ← https://rust-backend-prod-xxxx.up.railway.app
```

---

## ✅ 验证清单

部署后检查：

- [ ] 访问前端 URL → 看到用户列表
- [ ] 打开浏览器 DevTools (F12)
- [ ] Network 标签 → 查找 `/api/users` 请求
- [ ] 请求状态应为 200
- [ ] 能够添加/编辑/删除用户

---

## 🚨 常见问题

| 问题 | 原因 | 解决 |
|------|------|------|
| 前端 404 | Dockerfile构建失败 | 检查 Railway logs |
| API 404 | BACKEND_URL错误 | 检查环境变量设置 |
| API 连接失败 | 后端未部署 | 确保后端服务已启动 |
| CORS 错误 | 代理配置问题 | 检查 Nginx 代理规则 |

---

## 📚 详细文档

| 文件 | 用途 |
|------|------|
| [DEPLOYMENT_GUIDE.md](../DEPLOYMENT_GUIDE.md) | 详细部署步骤 |
| [RAILWAY_DEPLOYMENT.md](../RAILWAY_DEPLOYMENT.md) | Railway 完整教程 |
| README.md | 项目概览 |

---

## 💰 Railway 免费额度

- **计算**: $5/月 免费额度
- **存储**: 100 GB/月 免费额度
- 超出部分按需计费

---

## 🛠️ 技术栈

| 组件 | 技术 |
|------|------|
| 后端 | Rust + Axum |
| 前端 | React + TypeScript + Vite |
| 反向代理 | Nginx |
| 容器化 | Docker |
| 部署 | Railway |
| 通信 | HTTP/HTTPS + JSON |
