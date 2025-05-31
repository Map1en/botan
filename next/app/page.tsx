'use client';

import React from 'react';
import { TextField, Button, IconButton, Alert } from '@mui/material';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTheme } from '@/theme/ThemeContext';
import Brightness4Icon from '@mui/icons-material/Brightness4';
import Brightness7Icon from '@mui/icons-material/Brightness7';
import EmailOtpDialog from './components/EmailOtpDialog';
import TwoFactorDialog from './components/TwoFactorDialog';

interface LoginResponse {
  // 当登录成功时，返回用户信息
  id?: string;
  username?: string;
  displayName?: string;

  // 当需要双因素验证时的字段
  requiresTwoFactorAuth?: string[];
}

export default function Page() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  // 弹窗状态
  const [emailOtpOpen, setEmailOtpOpen] = useState(false);
  const [twoFactorOpen, setTwoFactorOpen] = useState(false);

  // 当前登录凭据（用于后续验证步骤）
  const [currentCredentials, setCurrentCredentials] = useState<{
    username: string;
    password: string;
    emailOtp?: string;
    twoFactorCode?: string;
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
    emailOtp?: string;
    twoFactorCode?: string;
  }) {
    try {
      setLoading(true);
      clearMessages();

      const result = (await invoke('auth_user', {
        credentials: {
          username: credentials.username,
          password: credentials.password,
        },
      })) as LoginResponse;

      console.log('Login response:', result);

      // 检查是否需要双因素验证
      if (
        result.requiresTwoFactorAuth &&
        result.requiresTwoFactorAuth.length > 0
      ) {
        // 更新当前凭据
        setCurrentCredentials(credentials);

        if (result.requiresTwoFactorAuth.includes('emailOtp')) {
          // 需要邮箱OTP验证
          setEmailOtpOpen(true);
        } else {
          // 需要其他2FA验证
          setTwoFactorOpen(true);
        }
      } else if (result.id) {
        // 登录成功 - 有用户ID表示成功
        setSuccess('登录成功！');
        console.log('Login successful:', result);

        // 清理状态
        setUsername('');
        setPassword('');
        setCurrentCredentials({ username: '', password: '' });
      } else {
        // 未知响应格式
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

  const handleEmailOtpSubmit = (otp: string) => {
    setEmailOtpOpen(false);
    performLogin({
      ...currentCredentials,
      emailOtp: otp,
    });
  };

  const handleTwoFactorSubmit = (code: string) => {
    setTwoFactorOpen(false);
    performLogin({
      ...currentCredentials,
      twoFactorCode: code,
    });
  };

  const handleDialogClose = () => {
    setEmailOtpOpen(false);
    setTwoFactorOpen(false);
    setLoading(false);
    setCurrentCredentials({ username: '', password: '' });
  };

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

      {/* 邮箱OTP验证弹窗 */}
      <EmailOtpDialog
        open={emailOtpOpen}
        onClose={handleDialogClose}
        onSubmit={handleEmailOtpSubmit}
        loading={loading}
      />

      {/* 2FA验证弹窗 */}
      <TwoFactorDialog
        open={twoFactorOpen}
        onClose={handleDialogClose}
        onSubmit={handleTwoFactorSubmit}
        loading={loading}
      />
    </div>
  );
}
