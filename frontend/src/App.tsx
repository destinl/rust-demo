import React, { useState, useEffect } from 'react'
import './App.css'

interface User {
  id: number
  name: string
  email: string
}

function App() {
  const [users, setUsers] = useState<User[]>([])
  const [name, setName] = useState('')
  const [email, setEmail] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchUsers = async () => {
    try {
      const response = await fetch('/api/users')
      if (!response.ok) throw new Error('Failed to fetch users')
      const data = await response.json()
      setUsers(data)
      setError(null)
    } catch (error) {
      console.error('Error fetching users:', error)
      setError('Failed to load users')
    }
  }

  const createUser = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name || !email) return
    
    setLoading(true)
    setError(null)
    try {
      const response = await fetch('/api/users', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, email })
      })
      if (!response.ok) throw new Error('Failed to create user')
      const newUser = await response.json()
      setUsers([...users, newUser])
      setName('')
      setEmail('')
    } catch (error) {
      console.error('Error creating user:', error)
      setError('Failed to create user')
    } finally {
      setLoading(false)
    }
  }

  const deleteUser = async (id: number) => {
    if (!confirm('确定要删除吗？')) return
    try {
      const response = await fetch(`/api/users/${id}`, { method: 'DELETE' })
      if (!response.ok) throw new Error('Failed to delete user')
      setUsers(users.filter(user => user.id !== id))
      setError(null)
    } catch (error) {
      console.error('Error deleting user:', error)
      setError('Failed to delete user')
    }
  }

  useEffect(() => {
    fetchUsers()
  }, [])

  return (
    <div className="app">
      <h1>🚀 Rust + React CRUD App</h1>
      
      {error && <div className="error">{error}</div>}
      
      <form onSubmit={createUser}>
        <input
          type="text"
          placeholder="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          required
        />
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
        />
        <button type="submit" disabled={loading}>
          {loading ? 'Creating...' : 'Create User'}
        </button>
      </form>

      <div className="users">
        <h2>Users ({users.length})</h2>
        {users.length === 0 && !loading && <p>No users yet. Create one!</p>}
        {users.map(user => (
          <div key={user.id} className="user-card">
            <h3>{user.name}</h3>
            <p>{user.email}</p>
            <button onClick={() => deleteUser(user.id)}>Delete</button>
          </div>
        ))}
      </div>
    </div>
  )
}

export default App
