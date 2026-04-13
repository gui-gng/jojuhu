import { create } from "zustand";
import type { User } from "@/lib/types";
import { setApiToken } from "@/lib/api";
import { setToken, getToken, removeToken } from "@/lib/stronghold";

interface AuthState {
  user: User | null;
  token: string | null;
  isLoading: boolean;
  isInitialized: boolean;
  setUser: (user: User | null) => void;
  setToken: (token: string | null) => void;
  login: (token: string, user: User) => Promise<void>;
  logout: () => Promise<void>;
  init: () => Promise<void>;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  token: null,
  isLoading: true,
  isInitialized: false,
  setUser: (user) => set({ user }),
  setToken: (token) => {
    setApiToken(token);
    set({ token });
  },
  login: async (token, user) => {
    await setToken(token);
    setApiToken(token);
    set({ token, user });
  },
  logout: async () => {
    await removeToken();
    setApiToken(null);
    set({ token: null, user: null });
  },
  init: async () => {
    try {
      const stored = await getToken();
      if (stored) {
        setApiToken(stored);
        set({ token: stored });
      }
    } finally {
      set({ isLoading: false, isInitialized: true });
    }
  },
}));
