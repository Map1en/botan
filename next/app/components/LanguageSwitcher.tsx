'use client';

import {
  IconButton,
  Menu,
  MenuItem,
  ListItemIcon,
  ListItemText,
} from '@mui/material';
import LanguageIcon from '@mui/icons-material/Language';
import { useState, useEffect } from 'react';

const languages = [
  { code: 'zh-CN', name: '中文', flag: '🇨🇳' },
  { code: 'en-US', name: 'English', flag: '🇺🇸' },
];

export default function LanguageSwitcher() {
  const [currentLocale, setCurrentLocale] = useState('zh-CN');
  const [isInitialized, setIsInitialized] = useState(false);
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  useEffect(() => {
    if (typeof window !== 'undefined' && !isInitialized) {
      const savedLocale = localStorage.getItem('locale') || 'zh-CN';
      setCurrentLocale(savedLocale);
      setIsInitialized(true);
    }
  }, [isInitialized]);

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  const switchLanguage = (locale: string) => {
    if (currentLocale !== locale) {
      localStorage.setItem('locale', locale);
      setCurrentLocale(locale);
      handleClose();

      window.dispatchEvent(
        new CustomEvent('languageChanged', {
          detail: { locale },
        }),
      );
    } else {
      handleClose();
    }
  };

  if (!isInitialized) {
    return (
      <IconButton color="inherit" size="small">
        <LanguageIcon />
      </IconButton>
    );
  }

  return (
    <>
      <IconButton
        color="inherit"
        size="small"
        onClick={handleClick}
        title="切换语言 / Switch Language"
        aria-controls={open ? 'language-menu' : undefined}
        aria-haspopup="true"
        aria-expanded={open ? 'true' : undefined}>
        <LanguageIcon />
      </IconButton>

      <Menu
        id="language-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        slotProps={{
          list: {
            'aria-labelledby': 'language-button',
          },
        }}
        transformOrigin={{ horizontal: 'right', vertical: 'top' }}
        anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}>
        {languages.map((language) => (
          <MenuItem
            key={language.code}
            onClick={() => switchLanguage(language.code)}
            selected={language.code === currentLocale}
            dense>
            <ListItemIcon sx={{ minWidth: 32 }}>
              <span style={{ fontSize: '18px' }}>{language.flag}</span>
            </ListItemIcon>
            <ListItemText primary={language.name} />
          </MenuItem>
        ))}
      </Menu>
    </>
  );
}
