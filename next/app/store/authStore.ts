import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';

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

interface AuthState {
  user: CurrentUser | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
}

interface AuthActions {
  login: (credentials: { username: string; password: string }) => Promise<any>;
  logout: () => void;
  setUser: (user: CurrentUser) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
  verify2FA: (
    credentials: { username: string; password: string },
    code: string,
    type: 'email' | '2fa',
  ) => Promise<any>;
}

export const useAuthStore = create<AuthState & AuthActions>()(
  persist(
    (set, get) => ({
      user: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,

      login: async (credentials) => {
        set({ isLoading: true, error: null });

        try {
          const result = await invoke('login', { credentials });
          return result;
        } catch (error: any) {
          set({ error: error.message || 'Login failed' });
          throw error;
        } finally {
          set({ isLoading: false });
        }
      },

      verify2FA: async (credentials, code, type) => {
        set({ isLoading: true, error: null });

        try {
          const codeData =
            type === 'email' ? { IsB: { code } } : { IsA: { code } };

          const result = (await invoke('verify2_fa', {
            twoFaType: type,
            code: codeData,
          })) as { success: boolean; data?: { verified: boolean } };

          if (result.success && result.data?.verified) {
            const loginResult = (await invoke('login', { credentials })) as {
              success: boolean;
              data?: CurrentUser;
            };
            if (loginResult.success && loginResult.data) {
              set({
                user: loginResult.data,
                isAuthenticated: true,
                error: null,
              });
            }
            return loginResult;
          }

          return result;
        } catch (error: any) {
          set({ error: error.message || '2FA verification failed' });
          throw error;
        } finally {
          set({ isLoading: false });
        }
      },

      logout: () => {
        set({
          user: null,
          isAuthenticated: false,
          error: null,
        });
      },

      setUser: (user: CurrentUser) => {
        set({
          user,
          isAuthenticated: true,
          error: null,
        });
      },

      setLoading: (loading: boolean) => {
        set({ isLoading: loading });
      },

      setError: (error: string | null) => {
        set({ error });
      },

      clearError: () => {
        set({ error: null });
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        user: state.user,
        isAuthenticated: state.isAuthenticated,
      }),
    },
  ),
);
