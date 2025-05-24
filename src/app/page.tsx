'use client';

import { TextField, Button } from '@mui/material';
import { useState } from 'react';

export default function Page() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  function login(info: { username: string; password: string }) {
    const username = encodeURIComponent(info.username);
    const password = encodeURIComponent(info.password);
    const auth = btoa(`${username}:${password}`);

    fetch('api/auth/user', {
      method: 'GET',
      headers: {
        Authorization: `Basic ${auth}`,
      },
    })
      .then((response) => response.json())
      .then((data) => {
        console.log('Success:', data);
      })
      .catch((error) => {
        console.error('Error:', error);
      });
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
