FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . ./
RUN npm run build

FROM nginx:alpine

# 复制构建后的应用
COPY --from=builder /app/dist /usr/share/nginx/html

# 简单的 Nginx 配置模板 - 仅提供静态文件
RUN cat > /etc/nginx/conf.d/default.conf.template <<'EOF'
server {
    listen ${PORT};
    root /usr/share/nginx/html;
    index index.html;
    
    # 日志配置
    error_log /var/log/nginx/error.log warn;
    access_log /var/log/nginx/access.log;
    
    # 提供静态文件和 SPA 路由支持
    location / {
        try_files $uri $uri/ /index.html;
    }
}
EOF

# 简单启动脚本 - 替换 PORT 变量（不替换 $uri 等 nginx 变量）
RUN cat > /docker-entrypoint-custom.sh <<'SCRIPT'
#!/bin/sh
PORT=${PORT:-5173}
echo "Starting nginx on port $PORT"
# 只替换 ${PORT}，不替换 $uri 等 nginx 变量
sed "s/\${PORT}/$PORT/g" /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf
exec "$@"
SCRIPT
RUN chmod +x /docker-entrypoint-custom.sh

EXPOSE 5173
ENTRYPOINT ["/docker-entrypoint-custom.sh"]
CMD ["nginx", "-g", "daemon off;"]