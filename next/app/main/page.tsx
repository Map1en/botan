'use client';

import React, { useEffect, useState } from 'react';
import {
  Typography,
  Paper,
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Accordion,
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
import DraggableFloatingBox, {
  FloatingActionButton,
} from '../components/FriendBox';
import FriendList from '../components/FriendList';
import AccountCircleIcon from '@mui/icons-material/AccountCircle';
import FriendsPage from '../components/FriendsPage';

export default function MainPage() {
  const { t } = useClientTranslations();
  const router = useRouter();
  const { user, logout, isAuthenticated } = useAuthStore();
  const { mode, setMode } = useTheme();
  const [showFloatingBox, setShowFloatingBox] = useState(false);
  const [showInfoPanel, setShowInfoPanel] = useState(true);

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
          {user?.currentAvatarThumbnailImageUrl ? (
            <img
              src={user.currentAvatarThumbnailImageUrl}
              alt="Avatar"
              className="mr-2 h-7 w-7 rounded-full object-cover"
            />
          ) : null}
          <Typography variant="h6" className="flex-1">
            {t('main.welcome')} {user?.displayName || user?.username}
          </Typography>

          <div className="flex items-center space-x-2">
            <IconButton
              onClick={() => setShowInfoPanel(!showInfoPanel)}
              className="transition-colors"
              sx={{
                color: mode === 'dark' ? 'white' : 'text.primary',
              }}>
              <AccountCircleIcon />
            </IconButton>

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
        {showInfoPanel && (
          <Paper elevation={3} className="p-6">
            <Typography variant="h6" className="mb-4">
              {t('main.userInfo')}
            </Typography>

            <div className="flex flex-col gap-6 md:flex-row">
              <div className="flex flex-col items-center md:items-start">
                <div className="mb-4 h-32 w-32 overflow-hidden rounded-full bg-gray-200">
                  <img
                    src={user?.currentAvatarThumbnailImageUrl}
                    alt="Avatar"
                    className="h-full w-full object-cover"
                    // onError={(e) => {
                    //   e.currentTarget.src = '/default-avatar.png';
                    // }}
                  />
                </div>
                <div className="text-center md:text-left">
                  <Typography variant="h5" className="mb-2 font-bold">
                    {user?.displayName}
                  </Typography>
                  <Typography
                    variant="body2"
                    color="textSecondary"
                    className="mb-1">
                    @{user?.username}
                  </Typography>
                  <Typography variant="body2" color="textSecondary">
                    ID: {user?.id}
                  </Typography>
                </div>
              </div>

              <div className="flex-1 space-y-4">
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  <div>
                    <Typography
                      variant="subtitle2"
                      className="mb-1 font-semibold">
                      {t('main.profile.accountStatus')}
                    </Typography>
                    <div className="flex items-center gap-2">
                      <div
                        className={`h-3 w-3 rounded-full ${
                          user?.status === 'active'
                            ? 'bg-green-500'
                            : 'bg-gray-500'
                        }`}
                      />
                      <Typography variant="body2">
                        {user?.status} ({user?.state})
                      </Typography>
                    </div>
                  </div>

                  <div>
                    <Typography
                      variant="subtitle2"
                      className="mb-1 font-semibold">
                      {t('main.profile.lastLogin')}
                    </Typography>
                    <Typography variant="body2">
                      {user?.last_login
                        ? new Date(user.last_login).toLocaleDateString()
                        : t('common.notAvailable')}
                    </Typography>
                  </div>

                  <div>
                    <Typography
                      variant="subtitle2"
                      className="mb-1 font-semibold">
                      {t('main.profile.joinDate')}
                    </Typography>
                    <Typography variant="body2">
                      {user?.date_joined
                        ? new Date(user.date_joined).toLocaleDateString()
                        : t('common.notAvailable')}
                    </Typography>
                  </div>

                  <div>
                    <Typography
                      variant="subtitle2"
                      className="mb-1 font-semibold">
                      {t('main.profile.platform')}
                    </Typography>
                    <Typography variant="body2">
                      {user?.last_platform || t('common.unknown')}
                    </Typography>
                  </div>
                </div>

                <div>
                  <Typography
                    variant="subtitle2"
                    className="mb-2 font-semibold">
                    {t('main.security.title')}
                  </Typography>
                  <div className="flex flex-wrap gap-2">
                    <div
                      className={`rounded-full px-3 py-1 text-xs ${
                        user?.emailVerified
                          ? 'bg-green-100 text-green-800'
                          : 'bg-red-100 text-red-800'
                      }`}>
                      {user?.emailVerified
                        ? t('main.security.emailVerified')
                        : t('main.security.emailNotVerified')}
                    </div>
                    <div
                      className={`rounded-full px-3 py-1 text-xs ${
                        user?.twoFactorAuthEnabled
                          ? 'bg-green-100 text-green-800'
                          : 'bg-gray-100 text-gray-800'
                      }`}>
                      {user?.twoFactorAuthEnabled
                        ? t('main.security.twoFactorEnabled')
                        : t('main.security.twoFactorDisabled')}
                    </div>
                    {user?.ageVerified && (
                      <div className="rounded-full bg-blue-100 px-3 py-1 text-xs text-blue-800">
                        {t('main.security.ageVerified')}
                      </div>
                    )}
                  </div>
                </div>

                {user?.bio && (
                  <div>
                    <Typography
                      variant="subtitle2"
                      className="mb-1 font-semibold">
                      {t('main.profile.bio')}
                    </Typography>
                    <Typography variant="body2">{user.bio}</Typography>
                  </div>
                )}

                <div>
                  <Typography
                    variant="subtitle2"
                    className="mb-1 font-semibold">
                    {t('main.profile.friends')}
                  </Typography>
                  <Typography variant="body2">
                    {t('common.active')}: {user?.activeFriends?.length || 0} |{' '}
                    {t('common.online')}: {user?.onlineFriends?.length || 0} |{' '}
                    {t('common.total')}: {user?.friends?.length || 0}
                  </Typography>
                </div>
              </div>
            </div>
          </Paper>
        )}
      </div>

      <FriendsPage />
      {/* 改设计了，暂时不显示 */}
      {/* {!showFloatingBox && (
        <FloatingActionButton onClick={() => setShowFloatingBox(true)} />
      )}

      {showFloatingBox && (
        <DraggableFloatingBox
          title={t('friend.title')}
          onClose={() => setShowFloatingBox(false)}
          initialPosition={{ x: 50, y: 150 }}>
          <FriendList />
        </DraggableFloatingBox>
      )} */}
    </Box>
  );
}
