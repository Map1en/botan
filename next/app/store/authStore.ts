import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';

interface UserPresence {
  groups: string[];
  id: string;
  instance: string;
  instanceType: string;
  platform: string;
  status: string;
  travelingToInstance: string;
  travelingToWorld: string;
  world: string;
}

interface CurrentUser {
  acceptedTOSVersion?: number;
  acceptedPrivacyVersion?: number;
  accountDeletionDate?: string | null;
  accountDeletionLog?: string | null;
  activeFriends?: string[];
  ageVerificationStatus?: string;
  ageVerified?: boolean;
  allowAvatarCopying?: boolean;
  badges?: any[];
  bio?: string;
  bioLinks?: string[];
  contentFilters?: string[];
  currentAvatar?: string;
  currentAvatarImageUrl?: string;
  currentAvatarThumbnailImageUrl?: string;
  currentAvatarTags?: string[];
  date_joined?: string;
  developerType?: string;
  displayName?: string;
  emailVerified?: boolean;
  fallbackAvatar?: string;
  friendGroupNames?: string[];
  friendKey?: string;
  friends?: string[];
  hasBirthday?: boolean;
  hideContentFilterSettings?: boolean;
  userLanguage?: string | null;
  userLanguageCode?: string | null;
  hasEmail?: boolean;
  hasLoggedInFromClient?: boolean;
  hasPendingEmail?: boolean;
  homeLocation?: string;
  id?: string;
  isAdult?: boolean;
  isBoopingEnabled?: boolean;
  isFriend?: boolean;
  last_activity?: string;
  last_login?: string;
  last_mobile?: string | null;
  last_platform?: string;
  obfuscatedEmail?: string;
  obfuscatedPendingEmail?: string;
  oculusId?: string;
  googleId?: string;
  googleDetails?: Record<string, any>;
  picoId?: string;
  viveId?: string;
  offlineFriends?: string[];
  onlineFriends?: string[];
  pastDisplayNames?: string[];
  presence?: UserPresence;
  platform_history?: string[];
  profilePicOverride?: string;
  profilePicOverrideThumbnail?: string;
  pronouns?: string;
  receiveMobileInvitations?: boolean;
  state?: string;
  status?: string;
  statusDescription?: string;
  statusFirstTime?: boolean;
  statusHistory?: string[];
  steamDetails?: Record<string, any>;
  steamId?: string;
  tags?: string[];
  twoFactorAuthEnabled?: boolean;
  twoFactorAuthEnabledDate?: string | null;
  unsubscribe?: boolean;
  updated_at?: string;
  userIcon?: string;
  username?: string;
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
