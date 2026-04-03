import axios, { AxiosInstance, AxiosError } from 'axios';
import { User, CreateUserRequest, UpdateUserRequest, HealthResponse } from './types';

class ApiClient {
  private client: AxiosInstance;

  constructor() {
    // 在开发环境使用代理，生产环境使用环境变量
    const baseURL = import.meta.env.DEV ? '/api' : import.meta.env.VITE_API_URL || 'http://localhost:3000';
    
    this.client = axios.create({
      baseURL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // 响应拦截器
    this.client.interceptors.response.use(
      (response) => response,
      (error: AxiosError) => {
        console.error('API Error:', error.message);
        return Promise.reject(error);
      }
    );
  }

  // 健康检查
  async healthCheck(): Promise<HealthResponse> {
    const response = await this.client.get('/health');
    return response.data;
  }

  // 获取所有用户
  async getUsers(): Promise<User[]> {
    const response = await this.client.get('/users');
    return response.data;
  }

  // 获取单个用户
  async getUser(id: number): Promise<User> {
    const response = await this.client.get(`/users/${id}`);
    return response.data;
  }

  // 创建用户
  async createUser(data: CreateUserRequest): Promise<User> {
    const response = await this.client.post('/users', data);
    return response.data;
  }

  // 更新用户
  async updateUser(id: number, data: UpdateUserRequest): Promise<User> {
    const response = await this.client.put(`/users/${id}`, data);
    return response.data;
  }

  // 删除用户
  async deleteUser(id: number): Promise<void> {
    await this.client.delete(`/users/${id}`);
  }
}

export const apiClient = new ApiClient();
