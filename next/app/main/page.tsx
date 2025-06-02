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
    <Box
      sx={{ flexGrow: 1, minHeight: '100vh', bgcolor: 'background.default' }}>
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
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            {t('main.welcome')} {user?.displayName || user?.username}
          </Typography>

          <LanguageSwitcher />

          <IconButton
            onClick={toggleTheme}
            sx={{
              ml: 1,
              color: mode === 'dark' ? 'white' : 'text.primary',
            }}>
            {mode === 'dark' ? <Brightness7Icon /> : <Brightness4Icon />}
          </IconButton>

          <IconButton
            onClick={handleLogout}
            sx={{
              ml: 1,
              color: mode === 'dark' ? 'white' : 'text.primary',
            }}>
            <LogoutIcon />
          </IconButton>
        </Toolbar>
      </AppBar>

      <Box sx={{ p: 3 }}>
        <Typography variant="h4" gutterBottom>
          {t('main.title')}
        </Typography>

        <Paper elevation={3} sx={{ p: 3, mt: 3 }}>
          <Typography variant="h6" gutterBottom>
            {t('main.userInfo')}
          </Typography>
          <Box
            sx={{
              backgroundColor: mode === 'dark' ? 'grey.800' : 'grey.100',
              p: 2,
              borderRadius: 1,
              maxHeight: 400,
              overflow: 'auto',
            }}>
            <pre
              style={{
                margin: 0,
                fontSize: '12px',
                fontFamily: 'monospace',
                whiteSpace: 'pre-wrap',
                wordBreak: 'break-word',
                color: mode === 'dark' ? '#ffffff' : '#000000',
              }}>
              {JSON.stringify(user, null, 2)}
            </pre>
          </Box>
        </Paper>

        <Box sx={{ mt: 3, display: 'flex', gap: 2 }}>
          <Button variant="outlined" onClick={handleLogout}>
            {t('main.logout')}
          </Button>
        </Box>
      </Box>
    </Box>
  );
}
