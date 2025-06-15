'use client';

import React, { useState, useMemo } from 'react';
import {
  Container,
  Box,
  Paper,
  Typography,
  Avatar,
  Grid,
  Button,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  Chip,
  Badge,
  TextField,
  InputAdornment,
  IconButton,
  styled,
} from '@mui/material';
import {
  ExpandMore as ExpandMoreIcon,
  People as PeopleIcon,
  Star as StarIcon,
  PlayCircle as PlayCircleIcon,
  PowerSettingsNew as PowerOffIcon,
  VideogameAsset as VideogameAssetIcon,
  DesktopWindows as DesktopWindowsIcon,
  Search as SearchIcon,
  FilterList as FilterListIcon,
} from '@mui/icons-material';
import { useFriendStore, Friend, FriendState } from '../store/friendStore';
import { useClientTranslations } from '../hooks/useClientTranslations';
import { useTheme } from '../../theme/ThemeContext';

// 状态徽章组件
const StyledBadge = styled(Badge)(({ theme, ownerState }) => ({
  '& .MuiBadge-badge': {
    backgroundColor: ownerState.status === 'online' ? '#22c55e' : '#6b7280',
    color: ownerState.status === 'online' ? '#22c55e' : '#6b7280',
    boxShadow: `0 0 0 2px ${theme.palette.background.paper}`,
    '&::after': {
      position: 'absolute',
      top: 0,
      left: 0,
      width: '100%',
      height: '100%',
      borderRadius: '50%',
      animation:
        ownerState.status === 'online'
          ? 'ripple 1.2s infinite ease-in-out'
          : 'none',
      border: '1px solid currentColor',
      content: '""',
    },
  },
  '@keyframes ripple': {
    '0%': { transform: 'scale(.8)', opacity: 1 },
    '100%': { transform: 'scale(2.4)', opacity: 0 },
  },
}));

// 好友卡片组件
function FriendCard({
  friend,
  inSameRoom,
}: {
  friend: Friend;
  inSameRoom: boolean;
}) {
  const { t } = useClientTranslations();
  const { mode } = useTheme();

  const statusColor =
    friend.state === 'online'
      ? inSameRoom
        ? 'status.sameRoom'
        : 'status.online'
      : 'status.offline';

  return (
    <Paper
      elevation={2}
      sx={{
        p: 2,
        display: 'flex',
        flexDirection: 'column',
        height: '100%',
        border: '1px solid',
        borderColor: mode === 'dark' ? '#374151' : '#e5e7eb',
        transition: 'all 0.2s ease-in-out',
        '&:hover': {
          transform: 'translateY(-4px)',
          boxShadow:
            '0 10px 15px -3px rgba(0, 0, 0, 0.2), 0 4px 6px -2px rgba(0, 0, 0, 0.1)',
          borderColor: 'primary.main',
        },
      }}>
      <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
        <StyledBadge
          overlap="circular"
          anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
          variant="dot"
          ownerState={{ status: friend.state }}>
          <Avatar
            src={friend.avatar}
            sx={{
              width: 48,
              height: 48,
              border: `2px solid`,
              borderColor: statusColor,
            }}
          />
        </StyledBadge>
        <Box sx={{ ml: 2, overflow: 'hidden' }}>
          <Typography variant="subtitle1" fontWeight="bold" noWrap>
            {friend.name}
            {friend.isVIP && (
              <Chip
                label="VIP"
                size="small"
                color="warning"
                sx={{ ml: 1, height: 20 }}
              />
            )}
          </Typography>
          <Typography
            variant="caption"
            sx={{
              color: statusColor,
              display: 'flex',
              alignItems: 'center',
            }}>
            {t(`friend.status.${friend.state}`)}
            {friend.pendingState && (
              <Chip
                label={friend.pendingState}
                size="small"
                sx={{ ml: 1, height: 20 }}
              />
            )}
          </Typography>
        </Box>
      </Box>

      <Box sx={{ flexGrow: 1, mb: 2, minHeight: '40px' }}>
        {friend.memo && (
          <Typography
            variant="body2"
            sx={{
              color: 'text.secondary',
              fontStyle: 'italic',
            }}
            noWrap>
            {friend.memo}
          </Typography>
        )}
        {friend.pendingOffline && (
          <Typography
            variant="caption"
            sx={{
              color: 'text.secondary',
              display: 'block',
              mt: 0.5,
            }}>
            {t('friend.pendingOffline')}:{' '}
            {new Date(friend.pendingOfflineTime).toLocaleTimeString()}
          </Typography>
        )}
      </Box>

      <Box sx={{ display: 'flex', gap: 1 }}>
        {friend.state === 'online' ? (
          inSameRoom ? (
            <Button
              variant="contained"
              fullWidth
              sx={{
                backgroundColor: 'status.sameRoom',
                '&:hover': { backgroundColor: '#16a34a' },
              }}>
              {t('friend.actions.interact')}
            </Button>
          ) : (
            <>
              <Button variant="contained" color="primary" fullWidth>
                {t('friend.actions.invite')}
              </Button>
              <Button variant="contained" color="secondary" fullWidth>
                {t('friend.actions.join')}
              </Button>
            </>
          )
        ) : (
          <Button variant="outlined" color="secondary" disabled fullWidth>
            {t('friend.status.offline')}
          </Button>
        )}
      </Box>
    </Paper>
  );
}

// 好友分类组件
function FriendCategory({
  title,
  icon: Icon,
  friends,
  color,
  defaultExpanded = false,
  inSameRoom = false,
}: {
  title: string;
  icon: React.ElementType;
  friends: Friend[];
  color: string;
  defaultExpanded?: boolean;
  inSameRoom?: boolean;
}) {
  const { t } = useClientTranslations();

  if (friends.length === 0) return null;

  return (
    <Accordion defaultExpanded={defaultExpanded}>
      <AccordionSummary
        expandIcon={<ExpandMoreIcon />}
        sx={{
          borderLeft: `4px solid ${color}`,
          minHeight: '48px',
          '& .MuiAccordionSummary-content': {
            my: 0.5,
          },
        }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
          <Icon sx={{ color, fontSize: '1.25rem' }} />
          <Typography
            variant="h6"
            sx={{
              fontSize: '1rem',
              lineHeight: 1.2,
              fontWeight: 600,
            }}>
            {`${title} (${friends.length})`}
          </Typography>
        </Box>
      </AccordionSummary>
      <AccordionDetails>
        <Grid container spacing={2}>
          {friends.map((friend) => (
            <Grid item key={friend.id} xs={12} sm={6} md={4} lg={3}>
              <FriendCard friend={friend} inSameRoom={inSameRoom} />
            </Grid>
          ))}
        </Grid>
      </AccordionDetails>
    </Accordion>
  );
}

// 主页面组件
export default function FriendsPage() {
  const { t } = useClientTranslations();
  const { mode } = useTheme();
  const { friends } = useFriendStore();
  const [searchTerm, setSearchTerm] = useState('');

  const filteredFriends = useMemo(() => {
    return friends.filter(
      (friend) =>
        friend.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        friend.nickName.toLowerCase().includes(searchTerm.toLowerCase()) ||
        friend.memo.toLowerCase().includes(searchTerm.toLowerCase()),
    );
  }, [friends, searchTerm]);

  // 分类好友
  const onlineFriends = filteredFriends.filter((f) => f.state === 'online');
  const activeFriends = filteredFriends.filter((f) => f.state === 'active');
  const offlineFriends = filteredFriends.filter((f) => f.state === 'offline');

  return (
    <Container
      disableGutters
      maxWidth={false}
      sx={{
        py: 2,
        px: { xs: 1.5, sm: 2, md: 2.5 },
      }}>
      <header>
        <Box
          sx={{
            display: 'flex',
            flexDirection: { xs: 'column', md: 'row' },
            justifyContent: 'space-between',
            alignItems: 'center',
            mb: 4,
            gap: 2,
          }}>
          <Typography variant="h4" component="h1">
            {t('friend.title')}
          </Typography>
          <Box
            sx={{
              display: 'flex',
              alignItems: 'center',
              gap: 2,
              width: { xs: '100%', md: 'auto' },
            }}>
            <TextField
              variant="outlined"
              size="small"
              placeholder={t('friend.searchPlaceholder')}
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <SearchIcon />
                  </InputAdornment>
                ),
                sx: { borderRadius: '8px' },
              }}
              sx={{ width: { xs: '100%', sm: '300px' } }}
            />
            <Button
              variant="contained"
              startIcon={<FilterListIcon />}
              sx={{ display: { xs: 'none', sm: 'flex' } }}>
              {t('friend.filter')}
            </Button>
          </Box>
        </Box>
      </header>

      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 3 }}>
        <FriendCategory
          title={t('friend.categories.online')}
          icon={PlayCircleIcon}
          friends={onlineFriends}
          color={mode === 'dark' ? '#22d3ee' : '#0284c7'}
          defaultExpanded
        />
        <FriendCategory
          title={t('friend.categories.active')}
          icon={PeopleIcon}
          friends={activeFriends}
          color={mode === 'dark' ? '#4ade80' : '#16a34a'}
          defaultExpanded
        />
        <FriendCategory
          title={t('friend.categories.offline')}
          icon={PowerOffIcon}
          friends={offlineFriends}
          color={mode === 'dark' ? '#6b7280' : '#4b5563'}
        />
      </Box>
    </Container>
  );
}
