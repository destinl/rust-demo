FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . ./
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html

# Nginx 配置模板 - 提供静态文件和 API 代理
RUN cat > /etc/nginx/conf.d/default.conf.template <<'EOF'
server {
    listen ${PORT};
    root /usr/share/nginx/html;
    index index.html;
    
    error_log /var/log/nginx/error.log warn;
    access_log /var/log/nginx/access.log;
    
    # SPA 静态文件路由
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # API 代理 - 代理到后端
    location /api/ {
        proxy_pass ${BACKEND_URL}/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_buffering off;
    }
}
EOF

# 启动脚本 - 替换 PORT 和 BACKEND_URL
RUN cat > /docker-entrypoint-custom.sh <<'SCRIPT'
#!/bin/sh
PORT=${PORT:-5173}
BACKEND_URL=${BACKEND_URL:-http://rust-backend:3000}

echo "[nginx] PORT=$PORT, BACKEND_URL=$BACKEND_URL"

sed -e "s|\${PORT}|$PORT|g" \
    -e "s|\${BACKEND_URL}|$BACKEND_URL|g" \
    /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf

exec "$@"
SCRIPT
RUN chmod +x /docker-entrypoint-custom.sh

EXPOSE 5173
ENTRYPOINT ["/docker-entrypoint-custom.sh"]
CMD ["nginx", "-g", "daemon off;"]