'use client';

import { TextField, Button, IconButton } from '@mui/material';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTheme } from '@/theme/ThemeContext';
import Brightness4Icon from '@mui/icons-material/Brightness4';
import Brightness7Icon from '@mui/icons-material/Brightness7';

export default function Page() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const { mode, setMode } = useTheme();

  const toggleTheme = () => {
    setMode(mode === 'light' ? 'dark' : 'light');
  };

  async function login(info: { username: string; password: string }) {
    try {
      const result = await invoke('auth_user', {
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
    <div className="min-h-screen p-8">
      <div className="mb-4 flex justify-end">
        <IconButton onClick={toggleTheme} color="inherit">
          {mode === 'dark' ? <Brightness7Icon /> : <Brightness4Icon />}
        </IconButton>
      </div>

      <div className="mx-auto max-w-md">
        <h1 className="mb-6 text-2xl font-bold">Login</h1>

        <div className="flex flex-col gap-4">
          <TextField
            label="Username"
            variant="outlined"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
          />
          <TextField
            label="Password"
            variant="outlined"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          <Button
            variant="contained"
            onClick={() => login({ username, password })}
            className="mt-4">
            Login
          </Button>
        </div>
      </div>
    </div>
  );
}
