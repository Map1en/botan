'use client';

import React from 'react';
import {
  TextField,
  Button,
  IconButton,
  Alert,
  Box,
  Typography,
  Paper,
} from '@mui/material';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useClientTranslations } from './hooks/useClientTranslations';
import { useTheme } from '@/theme/ThemeContext';
import Brightness4Icon from '@mui/icons-material/Brightness4';
import Brightness7Icon from '@mui/icons-material/Brightness7';
import TwoFactorDialog from './components/TwoFactorDialog';
import LanguageSwitcher from './components/LanguageSwitcher';

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

type ApiResponse<T> = {
  status: number;
  success: boolean;
  data: T | null;
  message?: string;
  error_details: { message: string; status_code?: number } | null;
};

type LoginResponse = ApiResponse<CurrentUser | RequiresTwoFactorAuth>;

interface Verify2FaResult {
  verified: boolean;
  enabled?: boolean;
}

interface Verify2FaEmailCodeResult {
  verified: boolean;
}

type EitherTwoFactorResultType = ApiResponse<
  Verify2FaResult | Verify2FaEmailCodeResult
>;

export default function Page() {
  const { t } = useClientTranslations();

  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [type, setType] = useState<'email' | '2fa'>('2fa');
  const [twoFactorOpen, setTwoFactorOpen] = useState(false);
  const [twoFactorError, setTwoFactorError] = useState('');

  const [responseData, setResponseData] = useState<any>(null);

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

  const clearTwoFactorError = () => {
    setTwoFactorError('');
  };

  async function performLogin(credentials: {
    username: string;
    password: string;
  }) {
    try {
      setLoading(true);
      clearMessages();
      setResponseData(null);

      const result = (await invoke('login', {
        credentials: {
          username: credentials.username,
          password: credentials.password,
        },
      })) as LoginResponse;

      console.log('Login response:', result);
      setResponseData(result);

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
          setSuccess(t('login.messages.success'));
          console.log('Login successful:', result.data);

          setUsername('');
          setPassword('');
          setCurrentCredentials({ username: '', password: '' });
        }
      } else {
        setError(result.message || t('login.messages.failed'));
      }
    } catch (error: any) {
      console.error('Login failed:', error);
      setError(error.message || t('login.messages.failed'));
    } finally {
      setLoading(false);
    }
  }

  const handleInitialLogin = () => {
    if (!username.trim() || !password.trim()) {
      setError(t('login.messages.invalidCredentials'));
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
    setTwoFactorError('');
    setCurrentCredentials({ username: '', password: '' });
  };

  async function performVerify2FA(code: string, type: 'email' | '2fa') {
    try {
      setLoading(true);
      clearMessages();
      clearTwoFactorError();

      const codeData = type === 'email' ? { IsB: { code } } : { IsA: { code } };

      const result = (await invoke('verify2_fa', {
        twoFaType: type,
        code: codeData,
      })) as EitherTwoFactorResultType;

      console.log('2FA verification response:', result);

      if (result.success && result.data?.verified) {
        console.log(
          '2FA verification successful, performing login with cookies...',
        );

        setTwoFactorOpen(false);
        setTwoFactorError('');

        const loginResult = (await invoke('login', {
          credentials: {
            username: currentCredentials.username,
            password: currentCredentials.password,
          },
        })) as LoginResponse;

        console.log('Final login response:', loginResult);
        setResponseData(loginResult);

        if (loginResult.success && loginResult.data) {
          setSuccess(t('login.messages.success'));
          console.log('Login successful:', loginResult.data);

          setUsername('');
          setPassword('');
          setCurrentCredentials({ username: '', password: '' });
        } else {
          setError(
            loginResult.message || t('login.messages.invalidResponseFormat'),
          );
        }
      } else {
        setTwoFactorError(result.message || t('twoFactor.messages.failed'));
      }
    } catch (error: any) {
      console.error('2FA verification failed:', error);
      setTwoFactorError(error.message || t('twoFactor.messages.error'));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="min-h-screen p-8">
      <div className="mb-4 flex items-center justify-end">
        <LanguageSwitcher />
        <IconButton
          onClick={toggleTheme}
          color="inherit"
          title={
            mode === 'dark' ? t('theme.toggleLight') : t('theme.toggleDark')
          }>
          {mode === 'dark' ? <Brightness7Icon /> : <Brightness4Icon />}
        </IconButton>
      </div>

      <div className="mx-auto max-w-4xl">
        <div className="grid grid-cols-1 gap-8 lg:grid-cols-2">
          <div>
            <h1 className="mb-6 text-2xl font-bold">{t('login.title')}</h1>

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
                label={t('login.username')}
                variant="outlined"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                disabled={loading}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    handleInitialLogin();
                  }
                }}
              />
              <TextField
                label={t('login.password')}
                variant="outlined"
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                disabled={loading}
                onKeyDown={(e) => {
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
                {loading ? t('login.loggingIn') : t('login.loginButton')}
              </Button>
            </div>
          </div>

          {responseData && (
            <div>
              <Typography variant="h6" gutterBottom>
                响应数据
              </Typography>
              <Paper
                elevation={3}
                sx={{
                  p: 2,
                  maxHeight: '600px',
                  overflow: 'auto',
                  backgroundColor: mode === 'dark' ? '#1e1e1e' : '#f5f5f5',
                }}>
                <pre
                  style={{
                    margin: 0,
                    fontSize: '12px',
                    fontFamily:
                      'Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
                    whiteSpace: 'pre-wrap',
                    wordBreak: 'break-word',
                  }}>
                  {JSON.stringify(responseData, null, 2)}
                </pre>
              </Paper>
            </div>
          )}
        </div>
      </div>

      <TwoFactorDialog
        open={twoFactorOpen}
        onClose={handleDialogClose}
        onSubmit={handleTwoFactorSubmit}
        loading={loading}
        type={type}
        error={twoFactorError}
        onClearError={clearTwoFactorError}
      />
    </div>
  );
}
