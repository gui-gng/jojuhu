import { fetch as tauriFetch } from "@tauri-apps/plugin-http";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:8080";

export const apiBaseUrl = API_BASE_URL;

let authToken: string | null = null;

export function setApiToken(token: string | null) {
  authToken = token;
}

export function getApiToken(): string | null {
  return authToken;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
  error?: string;
}

async function apiRequest<T>(
  method: string,
  endpoint: string,
  body?: unknown,
  options?: RequestInit
): Promise<T> {
  const url = `${API_BASE_URL}${endpoint}`;
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(options?.headers as Record<string, string>),
  };

  if (authToken) {
    headers["Authorization"] = `Bearer ${authToken}`;
  }

  const response = await tauriFetch(url, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
    ...options,
  });

  if (response.status === 204) {
    return undefined as T;
  }

  const json = (await response.json()) as ApiResponse<T>;

  if (!response.ok || !json.success) {
    const error = json.error || json.message || `HTTP ${response.status}`;
    throw new Error(error);
  }

  return json.data as T;
}

export const api = {
  get: <T>(endpoint: string, options?: RequestInit) =>
    apiRequest<T>("GET", endpoint, undefined, options),
  post: <T>(endpoint: string, body?: unknown, options?: RequestInit) =>
    apiRequest<T>("POST", endpoint, body, options),
  put: <T>(endpoint: string, body?: unknown, options?: RequestInit) =>
    apiRequest<T>("PUT", endpoint, body, options),
  delete: <T>(endpoint: string, options?: RequestInit) =>
    apiRequest<T>("DELETE", endpoint, undefined, options),
};
