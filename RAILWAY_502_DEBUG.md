# Railway 502 错误诊断指南

## 问题描述
前端返回 502 Bad Gateway，通常意味着 nginx 无法连接到后端服务。

## 🔍 第一步：验证后端 URL

### 在 Railway Dashboard 中查找正确的后端 URL

1. **打开 Railway Dashboard**: https://railway.app/dashboard
2. **找到你的项目**: `rust-demo`
3. **在左侧服务列表中找到后端服务**：
   - 可能叫 `rust-backend` 或其他名字
   - 点击它

4. **在右侧面板找到 Public URL**：
   - 查找 "Domains" 或 "Public URL"
   - 应该显示格式如：`https://rust-demo-production-XXXX.up.railway.app` 或 `https://rust-backend-XXXX.up.railway.app`
   - **完整复制这个 URL**

### ⚠️ 常见错误
- ❌ `http://rust-backend:3000` (Docker 内网地址，Railway 上不存在)
- ❌ `https://rust-demo-production.up.railway.app` (可能缺少服务后缀)
- ✅ `https://rust-demo-backend-production-XXXX.up.railway.app` (应该是这样)

---

## 🔍 第二步：测试后端 URL 是否可访问

### 通过浏览器测试
在浏览器中访问后端的健康检查端点：
```
https://[你的后端URL]/health
```

应该返回：
```json
{"status":"ok"}
```

如果返回 404 或其他错误，说明 URL 不对。

---

## 🔍 第三步：更新前端的 BACKEND_URL

1. **进入前端服务**：
   - Railway Dashboard → 左侧选择 `react-frontend` 或 `rust-demo-fe`

2. **进入 Variables**：
   - 点击服务 → 找到 "Variables" 选项卡

3. **更新 BACKEND_URL**：
   - 找到已存在的 `BACKEND_URL` 变量
   - **删除**旧的值
   - **输入**正确的后端 URL（从第一步复制）
   - **保存**

4. **等待重新部署**：
   - Railway 会自动重新部署前端
   - 在 Deployments 中查看进度
   - 完成后（2-3分钟）刷新前端页面

---

## 🆘 还是不行？检查以下几点

### 1️⃣ 后端服务是否在运行？
- Dashboard 中后端服务是否显示 "Running" 状态？
- 如果是 "Crashed" 或 "Exited"，需要检查后端的 Build Logs 和 Deploy Logs

### 2️⃣ BACKEND_URL 的格式
- 使用的是 **HTTPS** 还是 HTTP？ (必须用 HTTPS)
- URL 中有没有多余的 `/` 或 `/api` 后缀？ (不要加)
- URL 中有没有空格？

### 3️⃣ 在前端查看详细错误信息
- 打开浏览器 DevTools (F12)
- 进入 **Console** 标签
- 进入 **Network** 标签，找 `/api/users` 请求
- 查看请求的完整 URL、响应状态码和错误信息

### 4️⃣ 检查前端日志
- Railway Dashboard → `rust-demo-fe` 服务 → **Deploy Logs**
- 查看 nginx 的启动信息：
  ```
  [Nginx] ℹ️ Custom config: PORT=8080, BACKEND_URL=...
  ```
- BACKEND_URL 是否正确显示？

---

## 📋 完整检查清单

- [ ] 后端服务在 Railway 上状态为 "Running"
- [ ] 能访问后端的 `/health` 端点且返回 200
- [ ] 在浏览器测试：`https://[后端URL]/health`
- [ ] 前端服务的 `BACKEND_URL` 已更新为正确的 URL
- [ ] `BACKEND_URL` 使用 HTTPS(不是 HTTP)
- [ ] `BACKEND_URL` 没有 `/api` 后缀
- [ ] 前端已重新部署（查看 Deployments）
- [ ] 前端日志显示 `BACKEND_URL` 配置正确
- [ ] 刷新前端页面，正常显示用户列表

---

## 📝 示例

假设你的后端 URL 是：
```
https://rust-demo-backend-production-abc123.up.railway.app
```

则：
- ✅ BACKEND_URL 应该是：`https://rust-demo-backend-production-abc123.up.railway.app`
- ❌ 不要这样：`https://rust-demo-backend-production-abc123.up.railway.app/api`
- ❌ 不要这样：`http://rust-demo-backend-production-abc123.up.railway.app`

---

## 🆘 还需要帮助？

如果以上步骤都做了还是不行，请收集以下信息：

1. **后端服务的实际 Public URL** (从 Railway Dashboard 复制)
2. **前端 Deploy Logs** 中的 nginx 配置行
3. **浏览器 DevTools** 中 `/api/users` 请求的完整信息
4. **后端的 HTTP Logs** (如果可用)
