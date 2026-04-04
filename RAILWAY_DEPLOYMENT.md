# Railway 部署指南

## 前提条件
- 注册 Railway 账户 (https://railway.app)
- 安装 Railway CLI (可选)
- 项目已推送到GitHub

## 部署步骤

### 1. 在Railway创建项目
1. 访问 https://railway.app
2. 点击 "New Project" → "Deploy from GitHub repo"
3. 选择本项目

### 2. 部署后端服务 (Rust)

在Railway Dashboard中：
1. 添加新服务 → "Deploy from repo"
2. 选择本项目
3. **配置后端服务：**
   - **Name**: `rust-backend`
   - **Dockerfile path**: `backend/Dockerfile`
   - **Port**: `3000`
   - 在"Variables"中，可添加其他环境变量

4. 获取后端的公网URL：
   - 服务部署完成后，在"Network"标签中查看"Public URL"
   - 格式通常是: `https://rust-backend-production-xxxx.up.railway.app`

### 3. 部署前端服务 (React + Nginx)

1. **添加新服务** → "Deploy from repo"
2. **配置前端服务：**
   - **Name**: `react-frontend`
   - **Dockerfile path**: `frontend/Dockerfile`
   - **Port**: `5173`
   
3. **关键步骤 - 配置环境变量：**
   在Railway Dashboard的前端服务页面，进入"Variables"标签，添加：
   
   ```
   BACKEND_URL=https://<rust-backend-public-url>
   ```
   
   例如：
   ```
   BACKEND_URL=https://rust-backend-production-xxxx.up.railway.app
   ```

4. 前端部署完成后，在"Network"标签中获取公网URL

### 4. 前后端连接验证

部署完成后：
1. 访问前端公网URL
2. 打开浏览器开发者工具 (F12)
3. 在Network标签中检查API请求
4. 确保`/api/*`请求被正确代理到后端

## 一键部署方式（推荐）

创建 `railway.json` 配置文件，在项目根目录：

```json
{
  "services": [
    {
      "name": "rust-backend",
      "dockerfilePath": "backend/Dockerfile",
      "port": 3000
    },
    {
      "name": "react-frontend",
      "dockerfilePath": "frontend/Dockerfile",
      "port": 5173,
      "envVars": {
        "BACKEND_URL": "http://rust-backend:3000"
      }
    }
  ]
}
```

## 故障排除

### API 请求返回 CORS 错误
- 检查后端是否配置了 CORS 头
- 确保 `BACKEND_URL` 正确指向后端服务

### 前端无法连接到后端
1. 检查 Nginx 代理配置
2. 验证 `BACKEND_URL` 环境变量是否正确设置
3. 在浏览器开发者工具中查看具体错误

### 构建失败
- 检查 Dockerfile 中的路径是否正确
- 确保所有依赖文件已提交到git

## 环境特定配置

### Docker Compose (本地开发)
```bash
BACKEND_URL=http://rust-backend:3000 docker compose up
```

### Railway (生产)
前端服务环境变量设置为该后端服务的Public URL

## 推荐架构

```
┌─────────────────────────────────┐
│         Railway Edge             │
│    (Global Load Balancer)       │
└──────────┬──────────────────────┘
           │
    ┌──────┴──────┐
    │             │
┌───▼────┐    ┌──▼────┐
│Frontend │    │Backend│
│(Nginx)  │◄───┤(Axum) │
│5173     │    │3000   │
└────┬────┘    └───────┘
     │
  (服务间通信使用BACKEND_URL)
```

## 成本优化建议

- Railway有免费额度，包括一定的计算和存储资源
- 可根据流量需求选择不同的实例大小
- 建议启用"Autoscaling"处理流量波动
