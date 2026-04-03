import React, { useState, useEffect } from 'react';
import { User, CreateUserRequest, UpdateUserRequest } from './types';
import { apiClient } from './api';
import './App.css';

function App() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [editingUser, setEditingUser] = useState<User | null>(null);
  
  // 表单状态
  const [formData, setFormData] = useState<CreateUserRequest>({
    name: '',
    email: '',
  });

  // 加载用户列表
  const loadUsers = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await apiClient.getUsers();
      setUsers(data);
    } catch (err) {
      setError('加载用户失败');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  // 健康检查
  const checkHealth = async () => {
    try {
      const health = await apiClient.healthCheck();
      console.log('API 健康状态:', health);
    } catch (err) {
      console.error('API 不可用:', err);
      setError('无法连接到后端服务');
    }
  };

  useEffect(() => {
    loadUsers();
    checkHealth();
  }, []);

  // 创建用户
  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.name || !formData.email) {
      setError('请填写姓名和邮箱');
      return;
    }

    setLoading(true);
    setError(null);
    try {
      await apiClient.createUser(formData);
      setFormData({ name: '', email: '' });
      await loadUsers();
    } catch (err) {
      setError('创建用户失败');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  // 更新用户
  const handleUpdate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingUser) return;

    setLoading(true);
    setError(null);
    try {
      const updateData: UpdateUserRequest = {
        name: editingUser.name,
        email: editingUser.email,
      };
      await apiClient.updateUser(editingUser.id, updateData);
      setEditingUser(null);
      await loadUsers();
    } catch (err) {
      setError('更新用户失败');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  // 删除用户
  const handleDelete = async (id: number) => {
    if (!confirm('确定要删除这个用户吗？')) return;

    setLoading(true);
    setError(null);
    try {
      await apiClient.deleteUser(id);
      await loadUsers();
    } catch (err) {
      setError('删除用户失败');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  // 开始编辑
  const startEdit = (user: User) => {
    setEditingUser(user);
  };

  // 取消编辑
  const cancelEdit = () => {
    setEditingUser(null);
  };

  return (
    <div className="app">
      <header className="header">
        <h1>🚀 Rust CRUD 应用</h1>
        <p>使用 Rust + Axum 后端 + React + TypeScript 前端</p>
      </header>

      <div className="container">
        {error && (
          <div className="error-message">
            ⚠️ {error}
            <button onClick={() => setError(null)}>关闭</button>
          </div>
        )}

        {/* 创建/编辑表单 */}
        <div className="form-container">
          <h2>{editingUser ? '编辑用户' : '创建新用户'}</h2>
          <form onSubmit={editingUser ? handleUpdate : handleCreate}>
            <div className="form-group">
              <label htmlFor="name">姓名：</label>
              <input
                type="text"
                id="name"
                value={editingUser ? editingUser.name : formData.name}
                onChange={(e) => {
                  if (editingUser) {
                    setEditingUser({ ...editingUser, name: e.target.value });
                  } else {
                    setFormData({ ...formData, name: e.target.value });
                  }
                }}
                required
                placeholder="请输入姓名"
              />
            </div>

            <div className="form-group">
              <label htmlFor="email">邮箱：</label>
              <input
                type="email"
                id="email"
                value={editingUser ? editingUser.email : formData.email}
                onChange={(e) => {
                  if (editingUser) {
                    setEditingUser({ ...editingUser, email: e.target.value });
                  } else {
                    setFormData({ ...formData, email: e.target.value });
                  }
                }}
                required
                placeholder="请输入邮箱"
              />
            </div>

            <div className="form-buttons">
              <button type="submit" disabled={loading}>
                {loading ? '处理中...' : editingUser ? '更新用户' : '创建用户'}
              </button>
              {editingUser && (
                <button type="button" onClick={cancelEdit} className="cancel-btn">
                  取消编辑
                </button>
              )}
            </div>
          </form>
        </div>

        {/* 用户列表 */}
        <div className="users-container">
          <h2>用户列表 ({users.length})</h2>
          {loading && <div className="loading">加载中...</div>}
          
          {!loading && users.length === 0 && (
            <div className="empty-state">
              <p>暂无用户数据</p>
              <p>请创建第一个用户</p>
            </div>
          )}

          {users.length > 0 && (
            <div className="users-grid">
              {users.map((user) => (
                <div key={user.id} className="user-card">
                  <div className="user-info">
                    <h3>{user.name}</h3>
                    <p className="user-id">ID: {user.id}</p>
                    <p className="user-email">📧 {user.email}</p>
                  </div>
                  <div className="user-actions">
                    <button onClick={() => startEdit(user)} className="edit-btn">
                      ✏️ 编辑
                    </button>
                    <button onClick={() => handleDelete(user.id)} className="delete-btn">
                      🗑️ 删除
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* API 状态 */}
        <div className="api-status">
          <div className="status-indicator">
            <span className="status-dot"></span>
            <span>后端服务: 运行中</span>
          </div>
          <div className="api-info">
            <small>API 端点: /api/users, /api/health</small>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
