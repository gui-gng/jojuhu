import React, { useEffect } from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import App from "./App";
import "./styles.css";
import { Toaster } from "sonner";
import { useAuthStore } from "./store/auth";
import { useThemeStore } from "./store/theme";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

function Bootstrap() {
  const initAuth = useAuthStore((s) => s.init);
  const setTheme = useThemeStore((s) => s.setTheme);
  const theme = useThemeStore((s) => s.theme);

  useEffect(() => {
    initAuth();
    setTheme(theme);
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <App />
        <Toaster position="top-right" richColors />
      </BrowserRouter>
    </QueryClientProvider>
  );
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Bootstrap />
  </React.StrictMode>
);
