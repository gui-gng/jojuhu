import { create } from "zustand";
import { persist } from "zustand/middleware";

type Theme = "dark" | "light" | "system";

interface ThemeState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
  resolvedTheme: "dark" | "light";
}

function getSystemTheme(): "dark" | "light" {
  if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    return "dark";
  }
  return "light";
}

export const useThemeStore = create<ThemeState>()(
  persist(
    (set) => ({
      theme: "system",
      resolvedTheme: getSystemTheme(),
      setTheme: (theme) => {
        const resolved = theme === "system" ? getSystemTheme() : theme;
        const root = window.document.documentElement;
        root.classList.remove("light", "dark");
        root.classList.add(resolved);
        set({ theme, resolvedTheme: resolved });
      },
    }),
    {
      name: "jojuhu-theme",
      onRehydrateStorage: () => (state) => {
        if (state) {
          const resolved =
            state.theme === "system" ? getSystemTheme() : state.theme;
          const root = window.document.documentElement;
          root.classList.remove("light", "dark");
          root.classList.add(resolved);
          state.resolvedTheme = resolved;
        }
      },
    }
  )
);
