import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import Mock from 'mockjs';

type FriendState = 'online' | 'active' | 'offline';

interface Friend {
  id: string;
  state: FriendState;
  isVIP: boolean;
  ref?: any;
  name: string;
  memo: string;
  pendingOffline: boolean;
  pendingOfflineTime: string;
  pendingState: string;
  nickName: string;
  avatar: string;
}

interface FriendStoreState {
  friends: Friend[];
}

interface FriendStoreActions {
  getFriends: () => Friend[];
  setFriends: (friends: Friend[]) => void;
}

const generateMockFriends = (): Friend[] => {
  return Mock.mock({
    'friends|12': [
      {
        id: '@guid',
        'state|1': ['online', 'active', 'offline'],
        isVIP: '@boolean',
        ref: null,
        name: '@cname',
        memo: '@csentence(0, 20)',
        pendingOffline: '@boolean',
        pendingOfflineTime: '@datetime("yyyy-MM-dd HH:mm:ss")',
        'pendingState|1': ['', 'busy', 'away', 'donotdisturb'],
        nickName: '@cname',
        avatar: '@image("100x100")',
      },
    ],
  }).friends;
};

export const useFriendStore = create<FriendStoreState & FriendStoreActions>()(
  persist(
    (set, get) => ({
      friends: generateMockFriends(),

      getFriends: () => {
        return get().friends;
      },

      setFriends: (friends: Friend[]) => {
        set({ friends });
      },
    }),
    {
      name: 'friend-storage',
      partialize: (state) => ({
        friends: state.friends,
      }),
    },
  ),
);

export type { Friend, FriendState };
