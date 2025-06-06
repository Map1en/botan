'use client';

import React, { useState, useMemo } from 'react';
import {
  Typography,
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  Avatar,
  Box,
  Chip,
  TextField,
  Accordion,
  AccordionSummary,
  AccordionDetails,
} from '@mui/material';
import { ExpandMore as ExpandMoreIcon } from '@mui/icons-material';
import { useFriendStore, Friend, FriendState } from '../store/friendStore';
import { useClientTranslations } from '../hooks/useClientTranslations';
import { useTheme } from '../../theme/ThemeContext';

const getStateColor = (state: FriendState): string => {
  switch (state) {
    case 'online':
      return '#4caf50';
    case 'active':
      return '#ff9800';
    case 'offline':
      return '#9e9e9e';
    default:
      return '#9e9e9e';
  }
};

export default function FriendList() {
  const { t } = useClientTranslations();
  const { mode } = useTheme();
  const { friends } = useFriendStore();
  const [searchText, setSearchText] = useState('');

  const getStateLabel = (state: FriendState): string => {
    switch (state) {
      case 'online':
        return t('friend.status.online');
      case 'active':
        return t('friend.status.active');
      case 'offline':
        return t('friend.status.offline');
      default:
        return t('friend.status.unknown');
    }
  };

  const groupedFriends = useMemo(() => {
    const filtered = searchText.trim()
      ? friends.filter(
          (friend) =>
            friend.name.toLowerCase().includes(searchText.toLowerCase()) ||
            friend.nickName.toLowerCase().includes(searchText.toLowerCase()) ||
            friend.memo.toLowerCase().includes(searchText.toLowerCase()),
        )
      : friends;

    return {
      online: filtered.filter((friend) => friend.state === 'online'),
      active: filtered.filter((friend) => friend.state === 'active'),
      offline: filtered.filter((friend) => friend.state === 'offline'),
    };
  }, [friends, searchText]);

  const renderFriendList = (friendList: Friend[]) => {
    if (friendList.length === 0) {
      return (
        <div className="p-2 text-center">
          <Typography variant="body2" color="textSecondary">
            {searchText ? t('friend.noSearchResults') : t('friend.noFriends')}
          </Typography>
        </div>
      );
    }

    return (
      <List
        dense
        className="py-0"
        sx={{
          bgcolor: mode === 'dark' ? 'grey.800' : 'background.paper',
        }}>
        {friendList.map((friend) => (
          <ListItem
            key={friend.id}
            sx={{
              '&:hover': {
                bgcolor: mode === 'dark' ? 'grey.700' : 'grey.50',
              },
            }}>
            <ListItemAvatar>
              <Avatar src={friend.avatar} alt={friend.name} className="h-8 w-8">
                {friend.name.charAt(0)}
              </Avatar>
            </ListItemAvatar>
            <ListItemText
              primary={
                <div className="flex items-center gap-2">
                  <span className="text-sm font-medium">{friend.name}</span>
                  {friend.isVIP && (
                    <Chip
                      label="VIP"
                      size="small"
                      color="warning"
                      variant="outlined"
                      className="h-4 text-xs"
                    />
                  )}
                  <div
                    className="h-2 w-2 rounded-full"
                    style={{ backgroundColor: getStateColor(friend.state) }}
                    title={getStateLabel(friend.state)}
                  />
                </div>
              }
              secondary={
                <div className="space-y-0.5 text-xs">
                  <div>@{friend.nickName}</div>
                  {friend.memo && (
                    <div className="truncate text-gray-500 dark:text-gray-400">
                      {friend.memo}
                    </div>
                  )}
                </div>
              }
              slotProps={{
                primary: { component: 'div' },
                secondary: { component: 'div' },
              }}
            />
          </ListItem>
        ))}
      </List>
    );
  };

  return (
    <div className="flex h-full max-h-screen flex-col overflow-hidden rounded">
      <Box
        sx={{
          height: '100%',
          bgcolor: mode === 'dark' ? '#000000' : 'background.default',
          border: mode === 'dark' ? '1px solid #333' : '1px solid #e0e0e0',
        }}>
        <div className="flex-shrink-0">
          <Box>
            <TextField
              fullWidth
              size="small"
              placeholder={t('friend.searchPlaceholder')}
              value={searchText}
              onChange={(e) => setSearchText(e.target.value)}
              sx={{
                '& .MuiOutlinedInput-root': {
                  bgcolor: mode === 'dark' ? 'grey.800' : 'background.paper',
                },
              }}
            />
          </Box>
        </div>

        <div className="flex-1 overflow-auto">
          <Box>
            <Accordion
              defaultExpanded
              disableGutters
              sx={{
                bgcolor: mode === 'dark' ? 'grey.800' : 'background.paper',
                '&:before': { display: 'none' },
                boxShadow: 'none',
                borderBottom:
                  mode === 'dark' ? '1px solid #333' : '1px solid #e0e0e0',
              }}>
              <AccordionSummary
                expandIcon={<ExpandMoreIcon />}
                className="min-h-12"
                sx={{
                  bgcolor: mode === 'dark' ? 'grey.700' : 'grey.50',
                  '&.Mui-expanded': {
                    minHeight: 48,
                  },
                }}>
                <div className="flex items-center gap-2">
                  <div
                    className="h-3 w-3 rounded-full"
                    style={{ backgroundColor: getStateColor('online') }}
                  />
                  <Typography variant="subtitle2">
                    {t('friend.status.online')} ({groupedFriends.online.length})
                  </Typography>
                </div>
              </AccordionSummary>
              <AccordionDetails sx={{ p: 0 }}>
                {renderFriendList(groupedFriends.online)}
              </AccordionDetails>
            </Accordion>
            <Accordion
              defaultExpanded
              disableGutters
              sx={{
                bgcolor: mode === 'dark' ? 'grey.800' : 'background.paper',
                '&:before': { display: 'none' },
                boxShadow: 'none',
                borderBottom:
                  mode === 'dark' ? '1px solid #333' : '1px solid #e0e0e0',
              }}>
              <AccordionSummary
                expandIcon={<ExpandMoreIcon />}
                className="min-h-12"
                sx={{
                  bgcolor: mode === 'dark' ? 'grey.700' : 'grey.50',
                  '&.Mui-expanded': {
                    minHeight: 48,
                  },
                }}>
                <div className="flex items-center gap-2">
                  <div
                    className="h-3 w-3 rounded-full"
                    style={{ backgroundColor: getStateColor('active') }}
                  />
                  <Typography variant="subtitle2">
                    {t('friend.status.active')} ({groupedFriends.active.length})
                  </Typography>
                </div>
              </AccordionSummary>
              <AccordionDetails sx={{ p: 0 }}>
                {renderFriendList(groupedFriends.active)}
              </AccordionDetails>
            </Accordion>
            <Accordion
              defaultExpanded
              disableGutters
              sx={{
                bgcolor: mode === 'dark' ? 'grey.800' : 'background.paper',
                '&:before': { display: 'none' },
                boxShadow: 'none',
              }}>
              <AccordionSummary
                expandIcon={<ExpandMoreIcon />}
                className="min-h-12"
                sx={{
                  bgcolor: mode === 'dark' ? 'grey.700' : 'grey.50',
                  '&.Mui-expanded': {
                    minHeight: 48,
                  },
                }}>
                <div className="flex items-center gap-2">
                  <div
                    className="h-3 w-3 rounded-full"
                    style={{ backgroundColor: getStateColor('offline') }}
                  />
                  <Typography variant="subtitle2">
                    {t('friend.status.offline')} (
                    {groupedFriends.offline.length})
                  </Typography>
                </div>
              </AccordionSummary>
              <AccordionDetails sx={{ p: 0 }}>
                {renderFriendList(groupedFriends.offline)}
              </AccordionDetails>
            </Accordion>
          </Box>
        </div>
      </Box>
    </div>
  );
}
