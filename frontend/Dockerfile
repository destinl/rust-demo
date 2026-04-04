# 构建阶段
FROM node:18-alpine AS builder

WORKDIR /app

# 复制依赖文件（不再需要 frontend/ 前缀）
COPY package*.json ./
RUN npm ci

# 复制源代码并构建
COPY . ./

# 显示 TypeScript 版本用于调试
RUN npx tsc --version

# 构建（显示详细错误）
RUN npm run build

# 运行阶段 - 使用 nginx 服务静态文件
FROM nginx:alpine

# 复制构建产物到 nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# 复制 nginx 配置（不再需要 frontend/ 前缀）
COPY nginx.conf /etc/nginx/conf.d/default.conf

# 注意：如果你想让 nginx 监听 5173 端口（与 Vite 开发服务器一致）
# 需要同时修改 nginx.conf 中的 listen 指令
EXPOSE 5173

CMD ["nginx", "-g", "daemon off;"]