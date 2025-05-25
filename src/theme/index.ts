import { createTheme, ThemeOptions } from '@mui/material/styles';

// 定义主题类型
export type ThemeMode = 'light' | 'dark' | 'custom';

// 定义主题配置接口
interface ThemeConfig {
  name: ThemeMode;
  primary: string;
  secondary: string;
  background: string;
  text: string;
}

export const themeConfigs: Record<ThemeMode, ThemeConfig> = {
  light: {
    name: 'light',
    primary: '#1976d2', // Material-UI 默认主色
    secondary: '#dc004e', // Material-UI 默认次色
    background: '#ffffff',
    text: '#000000',
  },
  dark: {
    name: 'dark',
    primary: '#90caf9', // 暗色主题下的主色
    secondary: '#f48fb1', // 暗色主题下的次色
    background: '#121212',
    text: '#ffffff',
  },
  custom: {
    name: 'custom',
    primary: '#4caf50', // 自定义主题色
    secondary: '#ff9800',
    background: '#f5f5f5',
    text: '#333333',
  },
};

export const createAppTheme = (mode: ThemeMode) => {
  const config = themeConfigs[mode];

  const themeOptions: ThemeOptions = {
    palette: {
      mode: mode === 'dark' ? 'dark' : 'light',
      primary: {
        main: config.primary,
      },
      secondary: {
        main: config.secondary,
      },
      background: {
        default: config.background,
        paper: mode === 'dark' ? '#1e1e1e' : '#ffffff',
      },
      text: {
        primary: config.text,
      },
    },
    components: {
      MuiButton: {
        styleOverrides: {
          root: {
            borderRadius: '8px',
          },
        },
      },
    },
  };

  return createTheme(themeOptions);
};
