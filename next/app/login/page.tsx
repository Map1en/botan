'use client';

import React, { useState, useEffect } from 'react';
import {
  TextField,
  Button,
  Alert,
  Box,
  Typography,
  IconButton,
} from '@mui/material';
import { useRouter } from 'next/navigation';
import Brightness4Icon from '@mui/icons-material/Brightness4';
import Brightness7Icon from '@mui/icons-material/Brightness7';
import { useAuthStore } from '../store/authStore';
import { useClientTranslations } from '../hooks/useClientTranslations';
import { useTheme } from '../../theme/ThemeContext';
import TwoFactorDialog from '../components/TwoFactorDialog';
import LanguageSwitcher from '../components/LanguageSwitcher';

export default function LoginPage() {
  const { t } = useClientTranslations();
  const router = useRouter();
  const { mode, setMode } = useTheme();
  const {
    login,
    verify2FA,
    setUser,
    isLoading,
    error,
    setError,
    clearError,
    isAuthenticated,
  } = useAuthStore();

  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [twoFactorOpen, setTwoFactorOpen] = useState(false);
  const [twoFactorError, setTwoFactorError] = useState('');
  const [type, setType] = useState<'email' | '2fa'>('2fa');
  const [currentCredentials, setCurrentCredentials] = useState<{
    username: string;
    password: string;
  }>({ username: '', password: '' });

  // 如果已经登录，重定向到主页
  useEffect(() => {
    if (isAuthenticated) {
      router.push('/main');
    }
  }, [isAuthenticated, router]);

  const toggleTheme = () => {
    setMode(mode === 'light' ? 'dark' : 'light');
  };

  const handleLogin = async () => {
    if (!username.trim() || !password.trim()) {
      setError(t('login.messages.invalidCredentials'));
      return;
    }

    try {
      const credentials = {
        username: username.trim(),
        password: password.trim(),
      };

      const result = await login(credentials);

      if (result.success && result.data) {
        if (
          'requiresTwoFactorAuth' in result.data &&
          result.data.requiresTwoFactorAuth
        ) {
          const twoFactorData = result.data.requiresTwoFactorAuth;
          setCurrentCredentials(credentials);

          if (twoFactorData.includes('emailOtp')) {
            setType('email');
          } else {
            setType('2fa');
          }
          setTwoFactorOpen(true);
        } else {
          // 登录成功，设置用户信息并跳转
          setUser(result.data);
          router.push('/main');
        }
      } else {
        setError(result.message || t('login.messages.failed'));
      }
    } catch (error: any) {
      console.error('Login failed:', error);
    }
  };

  const handleTwoFactorSubmit = async (code: string) => {
    if (!code.trim()) {
      setTwoFactorError(t('twoFactor.messages.codeRequired'));
      return;
    }

    try {
      setTwoFactorError('');

      const result = await verify2FA(currentCredentials, code.trim(), type);

      if (result.success && result.data) {
        // 2FA验证成功，跳转到主页
        setTwoFactorOpen(false);
        router.push('/main');
      } else {
        setTwoFactorError(t('twoFactor.messages.invalidCode'));
      }
    } catch (error: any) {
      console.error('2FA verification failed:', error);
      setTwoFactorError(
        error.message || t('twoFactor.messages.verificationError'),
      );
    }
  };

  const handleTwoFactorClose = () => {
    setTwoFactorOpen(false);
    setTwoFactorError('');
    setCurrentCredentials({ username: '', password: '' });
  };

  return (
    <Box
      sx={{
        minHeight: '100vh',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        position: 'relative',
      }}>
      {/* 右上角的控制按钮 */}
      <Box
        sx={{
          position: 'absolute',
          top: 16,
          right: 16,
          display: 'flex',
          gap: 1,
        }}>
        <LanguageSwitcher />
        <IconButton
          onClick={toggleTheme}
          title={
            mode === 'dark' ? t('theme.toggleLight') : t('theme.toggleDark')
          }>
          {mode === 'dark' ? <Brightness7Icon /> : <Brightness4Icon />}
        </IconButton>
      </Box>

      {/* 登录表单 */}
      <Box sx={{ maxWidth: 400, width: '100%', p: 3 }}>
        <Typography
          variant="h5"
          component="h1"
          align="left"
          sx={{ mb: 3, fontWeight: 500 }}>
          {t('login.title')}
        </Typography>

        {error && (
          <Alert severity="error" sx={{ mb: 2 }} onClose={clearError}>
            {error}
          </Alert>
        )}

        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <TextField
            label={t('login.username')}
            variant="outlined"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            disabled={isLoading}
            onKeyDown={(e) => e.key === 'Enter' && handleLogin()}
            fullWidth
          />

          <TextField
            label={t('login.password')}
            variant="outlined"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={isLoading}
            onKeyDown={(e) => e.key === 'Enter' && handleLogin()}
            fullWidth
          />

          <Button
            variant="contained"
            onClick={handleLogin}
            disabled={isLoading || !username.trim() || !password.trim()}
            size="large"
            fullWidth>
            {isLoading ? t('login.loggingIn') : t('login.loginButton')}
          </Button>
        </Box>

        <TwoFactorDialog
          open={twoFactorOpen}
          onClose={handleTwoFactorClose}
          onSubmit={handleTwoFactorSubmit}
          loading={isLoading}
          type={type}
          error={twoFactorError}
          onClearError={() => setTwoFactorError('')}
        />
      </Box>
    </Box>
  );
}
