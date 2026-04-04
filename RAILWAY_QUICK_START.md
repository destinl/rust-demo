# Railway 部署快速参考卡

## 🎯 一句话总结
在 Railway 上分别部署前后端，通过 **环境变量 `BACKEND_URL`** 让前端 nginx 能代理 API 请求到后端。

⚠️ **关键：`BACKEND_URL` 必须是后端的公网 HTTPS URL，不是 `http://rust-backend:3000`**

---

## ⚡ 部署步骤（5分钟）

### Step 1️⃣: 推送代码到 GitHub
```bash
git push origin main
```

### Step 2️⃣: 在 Railway 部署后端

1. 登录 https://railway.app
2. 点击 **New Project** → **Deploy from GitHub repo**
3. 选择本项目 `rust-demo`
4. 等待自动部署完成

后端服务会自动部署：
- Dockerfile: `backend/Dockerfile`
- Port: 3000

### Step 3️⃣: 获取后端的 Public URL ⭐

部署完成后：
1. 在 Railway Dashboard 点击 `rust-backend` 服务
2. 在右侧面板找到 **"Public URL"** 或 **Domains**
3. **复制它**（格式如 `https://rust-backend-production-xxxx.up.railway.app`）

✅ 测试后端是否正常：访问 `https://[你的URL]/health`，应返回 `{"status":"ok"}`

### Step 4️⃣: 部署前端并配置 BACKEND_URL ⭐ 关键

1. 在 Railway Dashboard 添加新服务 → 选择本项目
2. 前端服务会自动部署
3. **进入前端服务 → Variables → 添加环境变量：**

```
BACKEND_URL=https://rust-backend-production-xxxx.up.railway.app
```

⚠️ **重要：** 
- 使用 **HTTPS**，不是 HTTP
- 完整 URL，不要加 `/api` 后缀
- 替换 `xxxx` 为你从 Step 3 复制的实际 URL

4. 保存，Railway 会自动重新部署前端（2-3分钟）

### Step 5️⃣: 验证应用

1. 获取前端的 Public URL（在 `react-frontend` 服务右侧）
2. 在浏览器访问该 URL
3. 应该能看到用户列表并能进行增删改查

---

## 🚨 常见问题和解决方案

### ❌ "Application failed to respond"

**原因**：`BACKEND_URL` 未设置或设置错误

**解决**：
1. ✅ 在前端服务的 Variables 中检查 `BACKEND_URL` 是否存在
2. ✅ 确保它是 HTTPS URL（来自 Step 3）
3. ✅ 没有多余的空格和后缀
4. ✅ 保存后等待 2-3 分钟重新部署

### ❌ 前端加载正常，但 API 调用失败（空列表/无数据）

**原因**：BACKEND_URL 设置错误或后端未启动

**检查**：
1. 在浏览器 DevTools → Network 标签
2. 找到 `/api/users` 请求
3. 查看它的完整 URL 和响应状态

**常见错误**：
- ❌ `http://rust-backend:3000` (内网地址，Railway 上不存在)
- ❌ `https://localhost:3000` (本地地址，Railway 上不存在)
- ✅ `https://rust-backend-production-xxxx.up.railway.app` （正确）

### ❌ 页面无法加载（404）

**原因**：前端 Dockerfile 构建失败或没有部署

**解决**：
1. 检查 Railway Dashboard 的 Logs
2. 查找 "error" 或 "failed" 关键字
3. 常见原因：`npm` 依赖不兼容、Node 版本问题

---

## 📋 部署检查清单

- [ ] 后端部署成功（显示 "Deployment Successful"）
- [ ] 后端 Public URL 已复制
- [ ] 前端 Variables 中设置了 `BACKEND_URL=https://...`
- [ ] 使用的是 HTTPS 协议，不是 HTTP
- [ ] 前端部署完成后等待了 2-3 分钟
- [ ] 前端 Public URL 在浏览器中能访问
- [ ] 页面加载正常，显示用户列表
- [ ] 能够添加/删除用户

---

## 🔄 本地开发 vs Railway 部署

| 环境 | BACKEND_URL | 说明 |
|------|-------------|------|
| Docker Compose (本地) | `http://rust-backend:3000` | 内网通信 |
| Railway (云) | `https://rust-backend-production-xxxx.up.railway.app` | 公网通信，必须是 HTTPS |

---

## 📝 完整的部署流程示意图

```
GitHub Repo
    ↓
Railway 自动检测 Dockerfile
    ↓
┌─────────────────────────────┐
│ 后端和前端同时部署构建       │
└──────┬──────────────────────┘
       ↓
┌─────────────────────────────┐
│ 后端启动 → 获得 Public URL    │
└──────┬──────────────────────┘
       ↓
┌──────────────────────────────┐
│ 前端 Variables 中设置:        │
│ BACKEND_URL=后端的 URL       │
│ ↓ 重新部署                   │
└──────┬───────────────────────┘
       ↓
┌──────────────────────────────┐
│ 前端启动 nginx 代理           │
│ 用户请求 /api/* → 代理到后端  │
└──────────────────────────────┘
```

---

## 🎓 环境变量说明

### 后端 (`rust-backend`)
- `RUST_LOG=info`: 日志等级
- `PORT`: Railway 自动分配（通常 3000）

### 前端 (`react-frontend`)
- `PORT`: Railway 自动分配（通常 5173）
- `BACKEND_URL`: **你必须手动设置** ← 这是关键！


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
