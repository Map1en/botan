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
        name: '@pick(["John 张", "Mary 李", "Tom 王", "Lisa 陈", "Mike 刘", "Emma 杨", "David 周", "Sarah 吴", "James 郑", "Linda 孙"])',
        memo: '@pick(["Working on project 正在处理项目", "In meeting 会议中", "Available 有空", "Busy with coding 正在编码", "Taking a break 休息中", "On vacation 度假中", "Focus mode 专注模式", "Team building 团建中", "Client call 客户通话", "Code review 代码审查"])',
        pendingOffline: '@boolean',
        pendingOfflineTime: '@datetime("yyyy-MM-dd HH:mm:ss")',
        'pendingState|1': ['', 'busy', 'away', 'donotdisturb'],
        nickName:
          '@pick(["小张 John", "小李 Mary", "小王 Tom", "小陈 Lisa", "小刘 Mike", "小杨 Emma", "小周 David", "小吴 Sarah", "小郑 James", "小孙 Linda"])',
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
