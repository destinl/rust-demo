# Railway 部署问题修复指南

## 🔴 问题原因分析

您遇到的 **405 Method Not Allowed** 和 **JSON 解析错误** 来自以下原因：

### 1️⃣ **前端无法正确连接到后端**
- 本地开发时，前端和后端通过 Docker 内网通信（`http://rust-backend:3000`）
- Railway 部署时，两个服务运行在不同容器中，必须通过**公网 URL** 通信
- 如果 `BACKEND_URL` 仍指向 `http://rust-backend:3000`，会导致 DNS 解析失败或返回错误页面

### 2️⃣ **PORT 环境变量配置不正确**
- Railway 自动为每个服务分配 `PORT` 环境变量
- 后端需要监听 Railway 分配的 PORT（不一定是 3000 或 8080）
- 前端 nginx 也需要监听正确的 PORT

### 3️⃣ **错误返回 HTML 而非 JSON**
- 当 nginx 无法连接到后端时，会返回错误页面（HTML）
- 当路由不匹配时，也会返回 HTML 错误页面

---

## ✅ 已修复的部分

### ✓ 后端 Dockerfile
- 添加了对 PORT 环境变量的支持
- 改用 `ENTRYPOINT` 确保环境变量被正确传递

### ✓ 前端 Dockerfile  
- 更新了启动脚本支持 `PORT` 环境变量
- nginx 将动态监听 Railway 分配的 PORT

### ✓ railway.toml
- clarified 需要在 Railway Dashboard 中配置 `BACKEND_URL`

---

## 🚀 Railway Dashboard 配置步骤

### **第1步：部署项目**

1. 推送代码到 GitHub
2. 在 Railway Dashboard 中创建新项目
3. 连接到您的 GitHub 仓库
4. 部署应该自动开始

### **第2步：配置后端环境变量**

进入 Railway Dashboard → `rust-backend` 服务 → Variables：

```
PORT = [Railway 自动分配，通常无需修改]
RUST_LOG = info
```

✅ 确保后端显示 "PORT: xxxx" 启动成功

### **第3步：获取后端公网 URL**

1. 在 Railway Dashboard 中，点击 `rust-backend` 服务
2. 在右侧面板找到 **"Public URL"** 或 **"Railway Domain"**
3. 格式通常为：`https://rust-demo-production-xxxx.up.railway.app`
4. **复制这个 URL**（完整的 HTTPS 链接）

⚠️ **确保后端服务暴露了公网访问权限**

### **第4步：配置前端环境变量**

进入 Railway Dashboard → `react-frontend` 服务 → Variables：

设置以下环境变量：

```
PORT = [Railway 自动分配，通常无需修改]
BACKEND_URL = https://rust-demo-production-xxxx.up.railway.app  # ← 使用第3步复制的URL
```

**重要**：使用 HTTPS 协议，不是 HTTP

### **第5步：重新部署**

- 修改环境变量后，Railway 会自动重新部署
- 等待前端和后端都显示 "Deployment Successful"

---

## 🧪 测试连接

### 测试后端健康检查

在浏览器中访问：
```
https://rust-demo-production-xxxx.up.railway.app/health
```

应该返回 JSON：
```json
{"status":"ok"}
```

### 测试前端

1. 访问前端公网 URL：`https://react-frontend-production-xxxx.up.railway.app`
2. 页面应该正常加载
3. 尝试创建、编辑、删除用户
4. 打开浏览器开发者工具 (F12) → Console，检查是否有错误

### 检查网络请求

在前端浏览器 DevTools → Network 标签：

- POST 请求应该发送到 `https://[BACKEND_URL]/api/users`
- 响应状态应该是 **200** 或 **201**（成功）
- 响应内容应该是 JSON，**不是 HTML**

---

## 🔧 常见问题排查

### ❌ 仍然出现 405 Method Not Allowed

**原因**：后端可能没有正确启动或路由配置有问题

**解决**：
1. 在 Railway Dashboard 中检查后端日志
2. 确保输出包含 "Server running on" 消息
3. 如果看到 panic 或 error，重新检查后端代码

### ❌ 返回 HTML 错误页面  

**原因**：nginx 无法连接到 `BACKEND_URL`

**解决**：
1. 确认 `BACKEND_URL` 环境变量设置正确（使用 HTTPS）
2. 测试后端公网 URL 是否可访问
3. 检查 CORS 是否配置正确

### ❌ 404 Not Found

**原因**：请求路径不正确

**解决**：
- 确认前端发送的请求是 `/api/users`（不是 `/users`）
- 检查 nginx 配置中 `proxy_pass` 是否正确

### ❌ CORS 错误

**原因**：跨域请求被阻止

**解决**：
- 后端已配置 `CorsLayer::new().allow_origin(Any)` 应该允许所有源
- 检查网络标签中是否有 Access-Control-Allow-Origin 响应头

### ❌ 前端无法加载

**原因**：PORT 配置问题或启动脚本失败

**解决**：
1. 检查前端日志中的启动信息
2. 查看是否有 nginx 错误消息
3. 确保 `PORT` 环境变量被正确读取

---

## 📝 完整检查清单

- [ ] 后端 Dockerfile 包含 ENTRYPOINT 支持 PORT 环境变量
- [ ] 前端 Dockerfile 启动脚本支持 PORT 和 BACKEND_URL 环境变量
- [ ] 在 Railway Dashboard 中设置了 `BACKEND_URL` 为后端的完整公网 HTTPS URL
- [ ] 后端 `/health` 端点可以通过公网访问并返回 JSON
- [ ] 前端可以访问并显示用户列表
- [ ] 可以创建、更新、删除用户
- [ ] 浏览器 DevTools 中没有 CORS 错误

---

## 🔗 参考资源

- [Railway PORT 环境变量](https://docs.railway.app/deploy/environment-variables)
- [Axum CORS 配置](https://docs.rs/tower-http/latest/tower_http/cors/)
- [Nginx 反向代理](https://nginx.org/en/docs/http/ngx_http_proxy_module.html)

---

**需要帮助？** 检查 Railway 控制面板中的部署日志，通常能找到问题原因。
