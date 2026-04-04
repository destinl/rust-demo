# 构建阶段
FROM node:18-alpine AS builder

WORKDIR /app

# 复制依赖文件
COPY package*.json ./
RUN npm ci

# 复制源代码并构建
COPY . ./

# 显示 TypeScript 版本用于调试
RUN npx tsc --version

# 构建时注入后端 API 地址
ARG VITE_API_URL
ENV VITE_API_URL=$VITE_API_URL

# 构建（显示详细错误）
RUN npm run build

# 运行阶段 - 使用 nginx 服务静态文件
FROM nginx:alpine

# 安装 envsubst 工具（Alpine 中默认已安装 gettext 包含 envsubst）
RUN apk add --no-cache gettext

# 复制构建产物到 nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# 复制 nginx 配置模板（注意文件名改为 .template）
COPY nginx.conf.template /etc/nginx/conf.d/default.conf.template

# 暴露端口
EXPOSE 5173

# 启动脚本：替换环境变量后启动 nginx
CMD sh -c "envsubst '\$BACKEND_URL' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"