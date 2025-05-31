'use client';

import React from 'react';
import { TextField, Button, IconButton, Alert } from '@mui/material';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTheme } from '@/theme/ThemeContext';
import Brightness4Icon from '@mui/icons-material/Brightness4';
import Brightness7Icon from '@mui/icons-material/Brightness7';
import TwoFactorDialog from './components/TwoFactorDialog';

interface CurrentUser {
  id?: string;
  username?: string;
  displayName?: string;
  bio?: string;
  currentAvatarThumbnailImageUrl?: string;
  status?: string;
  lastLogin?: string;
  emailVerified?: boolean;
  requiresTwoFactorAuth?: string[];
}

interface RequiresTwoFactorAuth {
  requiresTwoFactorAuth: string[];
}

type LoginResponse = CurrentUser | RequiresTwoFactorAuth;

interface Verify2FaResult {
  verified?: boolean;
  token?: string;
}

interface Verify2FaEmailCodeResult {
  verified?: boolean;
  token?: string;
}

type EitherTwoFactorResultType =
  | { IsA: Verify2FaResult }
  | { IsB: Verify2FaEmailCodeResult };

export default function Page() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [type, setType] = useState<'email' | '2fa'>('2fa');
  const [twoFactorOpen, setTwoFactorOpen] = useState(false);

  const [currentCredentials, setCurrentCredentials] = useState<{
    username: string;
    password: string;
  }>({ username: '', password: '' });

  const { mode, setMode } = useTheme();

  const toggleTheme = () => {
    setMode(mode === 'light' ? 'dark' : 'light');
  };

  const clearMessages = () => {
    setError('');
    setSuccess('');
  };

  async function performLogin(credentials: {
    username: string;
    password: string;
  }) {
    try {
      setLoading(true);
      clearMessages();

      const result = (await invoke('login', {
        credentials: {
          username: credentials.username,
          password: credentials.password,
        },
      })) as LoginResponse;

      console.log('Login response:', result.requiresTwoFactorAuth);

      if (result.requiresTwoFactorAuth) {
        const twoFactorData = result.requiresTwoFactorAuth;
        setCurrentCredentials(credentials);

        if (twoFactorData.includes('emailOtp')) {
          setType('email');
        } else {
          setType('2fa');
        }
        setTwoFactorOpen(true);
      } else if ('CurrentUser' in result) {
        const userData = result.CurrentUser;
        setSuccess('登录成功！');
        console.log('Login successful:', userData);

        setUsername('');
        setPassword('');
        setCurrentCredentials({ username: '', password: '' });
      } else {
        setError('登录响应格式异常');
      }
    } catch (error: any) {
      console.error('Login failed:', error);
      setError(error.message || '登录失败，请检查您的凭据');
    } finally {
      setLoading(false);
    }
  }

  const handleInitialLogin = () => {
    if (!username.trim() || !password.trim()) {
      setError('请输入用户名和密码');
      return;
    }

    performLogin({
      username: username.trim(),
      password: password.trim(),
    });
  };

  const handleTwoFactorSubmit = (code: string) => {
    performVerify2FA(code, type);
  };

  const handleDialogClose = () => {
    setTwoFactorOpen(false);
    setLoading(false);
    setCurrentCredentials({ username: '', password: '' });
  };

  async function performVerify2FA(code: string, type: 'email' | '2fa') {
    try {
      setLoading(true);
      clearMessages();

      const codeData = type === 'email' ? { IsB: { code } } : { IsA: { code } };

      const result = (await invoke('verify2_fa', {
        twoFaType: type,
        code: codeData,
      })) as EitherTwoFactorResultType;

      console.log('2FA verification response:', result);

      let verified = false;
      if ('IsA' in result) {
        verified = result.IsA.verified || false;
      } else if ('IsB' in result) {
        verified = result.IsB.verified || false;
      }

      if (verified) {
        console.log(
          '2FA verification successful, performing login with cookies...',
        );

        const loginResult = (await invoke('login', {
          credentials: {
            username: currentCredentials.username,
            password: currentCredentials.password,
          },
        })) as LoginResponse;

        console.log('Final login response:', loginResult);

        if ('CurrentUser' in loginResult) {
          const userData = loginResult.CurrentUser;
          setSuccess('登录成功！');
          console.log('Login successful:', userData);

          setUsername('');
          setPassword('');
          setCurrentCredentials({ username: '', password: '' });
          setTwoFactorOpen(false);
        } else if ('RequiresTwoFactorAuth' in loginResult) {
          setError('需要额外的验证步骤');
        } else {
          setError('登录响应格式异常');
        }
      } else {
        setError('验证失败，请检查验证码');
      }
    } catch (error: any) {
      console.error('2FA verification failed:', error);
      setError(error.message || '验证失败，请重试');
    } finally {
      setLoading(false);
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
        <h1 className="mb-6 text-2xl font-bold">登录</h1>

        {error && (
          <Alert severity="error" sx={{ mb: 2 }} onClose={clearMessages}>
            {error}
          </Alert>
        )}

        {success && (
          <Alert severity="success" sx={{ mb: 2 }} onClose={clearMessages}>
            {success}
          </Alert>
        )}

        <div className="flex flex-col gap-4">
          <TextField
            label="用户名/邮箱"
            variant="outlined"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            disabled={loading}
            onKeyPress={(e) => {
              if (e.key === 'Enter') {
                handleInitialLogin();
              }
            }}
          />
          <TextField
            label="密码"
            variant="outlined"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={loading}
            onKeyPress={(e) => {
              if (e.key === 'Enter') {
                handleInitialLogin();
              }
            }}
          />
          <Button
            variant="contained"
            onClick={handleInitialLogin}
            disabled={loading || !username.trim() || !password.trim()}
            className="mt-4">
            {loading ? '登录中...' : '登录'}
          </Button>
        </div>
      </div>

      <TwoFactorDialog
        open={twoFactorOpen}
        onClose={handleDialogClose}
        onSubmit={handleTwoFactorSubmit}
        loading={loading}
      />
    </div>
  );
}
