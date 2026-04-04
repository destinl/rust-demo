#!/bin/bash

# 本地开发环境测试脚本
# 用法：bash test-api.sh

BACKEND_URL="${BACKEND_URL:-http://localhost:3000}"
FRONTEND_URL="${FRONTEND_URL:-http://localhost:5173}"

echo "🧪 开始 API 测试..."
echo "================================"
echo "后端 URL: $BACKEND_URL"
echo "前端 URL: $FRONTEND_URL"
echo "================================"
echo ""

# 测试 1：健康检查
echo "📌 测试 1: 后端健康检查"
HEALTH=$(curl -s -w "\n%{http_code}" "$BACKEND_URL/health")
HTTP_CODE=$(echo "$HEALTH" | tail -n1)
BODY=$(echo "$HEALTH" | head -n-1)

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ 健康检查成功 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
else
    echo "❌ 健康检查失败 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
    exit 1
fi
echo ""

# 测试 2：获取用户列表
echo "📌 测试 2: 获取用户列表 (GET /api/users)"
USERS=$(curl -s -w "\n%{http_code}" "$BACKEND_URL/users")
HTTP_CODE=$(echo "$USERS" | tail -n1)
BODY=$(echo "$USERS" | head -n-1)

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ 获取用户成功 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
else
    echo "❌ 获取用户失败 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
fi
echo ""

# 测试 3：创建用户
echo "📌 测试 3: 创建新用户 (POST /api/users)"
CREATE=$(curl -s -w "\n%{http_code}" -X POST "$BACKEND_URL/users" \
  -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com"}')

HTTP_CODE=$(echo "$CREATE" | tail -n1)
BODY=$(echo "$CREATE" | head -n-1)

if [ "$HTTP_CODE" = "201" ] || [ "$HTTP_CODE" = "200" ]; then
    echo "✅ 创建用户成功 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
    
    # 从响应中提取用户 ID
    USER_ID=$(echo "$BODY" | grep -o '"id":[0-9]*' | head -1 | cut -d: -f2)
    echo "   用户 ID: $USER_ID"
else
    echo "❌ 创建用户失败 (HTTP $HTTP_CODE)"
    echo "   响应: $BODY"
fi
echo ""

# 测试 4：获取特定用户
if [ -n "$USER_ID" ]; then
    echo "📌 测试 4: 获取特定用户 (GET /api/users/$USER_ID)"
    GET_USER=$(curl -s -w "\n%{http_code}" "$BACKEND_URL/users/$USER_ID")
    HTTP_CODE=$(echo "$GET_USER" | tail -n1)
    BODY=$(echo "$GET_USER" | head -n-1)
    
    if [ "$HTTP_CODE" = "200" ]; then
        echo "✅ 获取特定用户成功 (HTTP $HTTP_CODE)"
        echo "   响应: $BODY"
    else
        echo "❌ 获取特定用户失败 (HTTP $HTTP_CODE)"
        echo "   响应: $BODY"
    fi
    echo ""
    
    # 测试 5：更新用户
    echo "📌 测试 5: 更新用户 (PUT /api/users/$USER_ID)"
    UPDATE=$(curl -s -w "\n%{http_code}" -X PUT "$BACKEND_URL/users/$USER_ID" \
      -H "Content-Type: application/json" \
      -d '{"name":"Updated User","email":"updated@example.com"}')
    
    HTTP_CODE=$(echo "$UPDATE" | tail -n1)
    BODY=$(echo "$UPDATE" | head -n-1)
    
    if [ "$HTTP_CODE" = "200" ]; then
        echo "✅ 更新用户成功 (HTTP $HTTP_CODE)"
        echo "   响应: $BODY"
    else
        echo "❌ 更新用户失败 (HTTP $HTTP_CODE)"
        echo "   响应: $BODY"
    fi
    echo ""
    
    # 测试 6：删除用户
    echo "📌 测试 6: 删除用户 (DELETE /api/users/$USER_ID)"
    DELETE=$(curl -s -w "\n%{http_code}" -X DELETE "$BACKEND_URL/users/$USER_ID")
    HTTP_CODE=$(echo "$DELETE" | tail -n1)
    
    if [ "$HTTP_CODE" = "204" ]; then
        echo "✅ 删除用户成功 (HTTP $HTTP_CODE)"
    else
        echo "❌ 删除用户失败 (HTTP $HTTP_CODE)"
        echo "   响应: $DELETE"
    fi
    echo ""
fi

echo "================================"
echo "✅ API 测试完成！"
echo ""
echo "💡 如果所有测试都通过，可以访问:"
echo "   $FRONTEND_URL"
