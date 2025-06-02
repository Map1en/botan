'use client';

import React, { useEffect } from 'react';
import {
  Typography,
  Paper,
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Button,
} from '@mui/material';
import { useRouter } from 'next/navigation';
import LogoutIcon from '@mui/icons-material/Logout';
import { useAuthStore } from '../store/authStore';
import { useClientTranslations } from '../hooks/useClientTranslations';
import { useTheme } from '../../theme/ThemeContext';
import Brightness4Icon from '@mui/icons-material/Brightness4';
import Brightness7Icon from '@mui/icons-material/Brightness7';
import LanguageSwitcher from '../components/LanguageSwitcher';

export default function MainPage() {
  const { t } = useClientTranslations();
  const router = useRouter();
  const { user, logout, isAuthenticated } = useAuthStore();
  const { mode, setMode } = useTheme();

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  const handleLogout = () => {
    logout();
    router.push('/login');
  };

  const toggleTheme = () => {
    setMode(mode === 'light' ? 'dark' : 'light');
  };

  if (!isAuthenticated) {
    return null;
  }

  return (
    <Box className="min-h-screen flex-1" sx={{ bgcolor: 'background.default' }}>
      <AppBar
        position="static"
        sx={{
          bgcolor: mode === 'dark' ? 'grey.900' : 'white',
          color: mode === 'dark' ? 'white' : 'text.primary',
          boxShadow:
            mode === 'dark'
              ? '0 2px 4px rgba(255,255,255,0.1)'
              : '0 2px 4px rgba(0,0,0,0.1)',
        }}>
        <Toolbar className="px-4 sm:px-6">
          <Typography variant="h6" className="flex-1">
            {t('main.welcome')} {user?.displayName || user?.username}
          </Typography>

          <div className="flex items-center space-x-2">
            <LanguageSwitcher />

            <IconButton
              onClick={toggleTheme}
              className="transition-colors"
              sx={{
                color: mode === 'dark' ? 'white' : 'text.primary',
              }}>
              {mode === 'dark' ? <Brightness7Icon /> : <Brightness4Icon />}
            </IconButton>

            <IconButton
              onClick={handleLogout}
              className="transition-colors"
              sx={{
                color: mode === 'dark' ? 'white' : 'text.primary',
              }}>
              <LogoutIcon />
            </IconButton>
          </div>
        </Toolbar>
      </AppBar>

      <div className="p-6">
        <Typography variant="h4" className="mb-8">
          {t('main.title')}
        </Typography>

        <Paper elevation={3} className="mt-6 p-6">
          <Typography variant="h6" className="mb-4">
            {t('main.userInfo')}
          </Typography>

          <div
            className="max-h-96 overflow-auto rounded-lg p-4"
            style={{
              backgroundColor: mode === 'dark' ? '#424242' : '#f5f5f5',
            }}>
            <pre
              className="font-mono text-xs break-words whitespace-pre-wrap"
              style={{
                margin: 0,
                color: mode === 'dark' ? '#ffffff' : '#000000',
              }}>
              {JSON.stringify(user, null, 2)}
            </pre>
          </div>
        </Paper>

        <div className="mt-6 flex flex-wrap gap-4">
          <Button
            variant="outlined"
            onClick={handleLogout}
            className="transition-transform hover:scale-105">
            {t('main.logout')}
          </Button>
        </div>
      </div>
    </Box>
  );
}
