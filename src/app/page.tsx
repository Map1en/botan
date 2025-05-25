'use client';

import { TextField, Button } from '@mui/material';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export default function Page() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  async function login(info: { username: string; password: string }) {
    try {
      const result = await invoke('mock_login', {
        credentials: {
          username: info.username,
          password: info.password,
        },
      });
      console.log('Login successful:', result);
    } catch (error) {
      console.error('Login failed:', error);
    }
  }

  return (
    <div className="m-50">
      <h1>login</h1>

      <div className="flex flex-col gap-4">
        <TextField
          id="outlined-basic"
          label="Username"
          variant="outlined"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <TextField
          id="outlined-basic"
          label="Password"
          variant="outlined"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <Button
          variant="contained"
          onClick={() => login({ username, password })}>
          login
        </Button>
      </div>
    </div>
  );
}
