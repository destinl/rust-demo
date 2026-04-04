FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . ./

# 构建参数（可选，用于构建时替换）
ARG VITE_API_URL
ENV VITE_API_URL=$VITE_API_URL

RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html

# 创建启动脚本，动态生成 Nginx 配置
RUN cat > /docker-entrypoint.sh <<'EOF'
#!/bin/sh
set -e

PORT=${PORT:-5173}
BACKEND_URL=${BACKEND_URL:-http://rust-backend:3000}

echo "[Nginx] PORT=$PORT"
echo "[Nginx] BACKEND_URL=$BACKEND_URL"

cat > /etc/nginx/conf.d/default.conf <<NGINX_CONF
server {
    listen $PORT;
    error_log /var/log/nginx/error.log warn;
    access_log /var/log/nginx/access.log;
    
    location / {
        root /usr/share/nginx/html;
        index index.html;
        try_files \$uri \$uri/ /index.html;
    }
    
    location /api/ {
        proxy_pass $BACKEND_URL/;
        proxy_connect_timeout 90s;
        proxy_send_timeout 90s;
        proxy_read_timeout 90s;
        proxy_ssl_verify off;
        proxy_ssl_session_reuse on;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_buffering off;
    }
}
NGINX_CONF

exec "$@"
EOF

RUN chmod +x /docker-entrypoint.sh

EXPOSE 5173

ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["nginx", "-g", "daemon off;"]