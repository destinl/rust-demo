// 用户接口
export interface User {
  id: number;
  name: string;
  email: string;
}

// 创建用户请求
export interface CreateUserRequest {
  name: string;
  email: string;
}

// 更新用户请求
export interface UpdateUserRequest {
  name: string;
  email: string;
}

// API 响应
export interface ApiResponse<T> {
  data?: T;
  error?: string;
}

// 健康检查响应
export interface HealthResponse {
  status: string;
}
