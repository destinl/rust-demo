import React, { useState, useEffect } from 'react';
import './App.css';

interface User {
  id: number;
  name: string;
  email: string;
}

// 获取 API 基础 URL
const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

function App() {
  const [users, setUsers] = useState<User[]>([]);
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [editingUser, setEditingUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchUsers = async () => {
    setLoading(true);
    try {
      const response = await fetch(`${API_BASE_URL}/api/users`);
      if (!response.ok) throw new Error('Failed to fetch');
      const data = await response.json();
      setUsers(data);
      setError(null);
    } catch (err) {
      setError('无法加载用户数据');
      console.error('Fetch error:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (editingUser) {
        const response = await fetch(`${API_BASE_URL}/api/users/${editingUser.id}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ name, email }),
        });
        if (!response.ok) throw new Error('Update failed');
      } else {
        const response = await fetch(`${API_BASE_URL}/api/users`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ name, email }),
        });
        if (!response.ok) throw new Error('Create failed');
      }
      setName('');
      setEmail('');
      setEditingUser(null);
      await fetchUsers();
    } catch (err) {
      setError(editingUser ? '更新用户失败' : '创建用户失败');
      console.error('Submit error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: number) => {
    if (!window.confirm('确定要删除这个用户吗？')) return;
    try {
      const response = await fetch(`${API_BASE_URL}/api/users/${id}`, { 
        method: 'DELETE' 
      });
      if (!response.ok) throw new Error('Delete failed');
      await fetchUsers();
    } catch (err) {
      setError('删除用户失败');
      console.error('Delete error:', err);
    }
  };

  const startEdit = (user: User) => {
    setEditingUser(user);
    setName(user.name);
    setEmail(user.email);
  };

  const cancelEdit = () => {
    setEditingUser(null);
    setName('');
    setEmail('');
  };

  return (
    <div className="app">
      <header className="header">
        <h1>🚀 Rust + React CRUD 应用</h1>
        <p>后端: Rust + Axum | 前端: React + TypeScript</p>
      </header>

      <div className="container">
        {error && <div className="error-message">⚠️ {error}</div>}

        <div className="form-container">
          <h2>{editingUser ? '编辑用户' : '创建新用户'}</h2>
          <form onSubmit={handleSubmit}>
            <input
              type="text"
              placeholder="姓名"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
            <input
              type="email"
              placeholder="邮箱"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
            <button type="submit" disabled={loading}>
              {loading ? '处理中...' : editingUser ? '更新用户' : '创建用户'}
            </button>
            {editingUser && <button type="button" onClick={cancelEdit}>取消</button>}
          </form>
        </div>

        <div className="users-container">
          <h2>用户列表 ({users.length})</h2>
          {loading && <div className="loading">加载中...</div>}
          {users.length === 0 && !loading && <div className="empty">暂无用户</div>}
          <div className="users-grid">
            {users.map(user => (
              <div key={user.id} className="user-card">
                <h3>{user.name}</h3>
                <p className="email">{user.email}</p>
                <div className="actions">
                  <button onClick={() => startEdit(user)}>编辑</button>
                  <button onClick={() => handleDelete(user.id)}>删除</button>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;