import { useEffect, useRef, useCallback } from "react";
import { useAuthStore } from "@/store/auth";
import type { WebSocketMessage } from "@/lib/types";

export function useWebSocket(
  onMessage: (msg: WebSocketMessage) => void,
  enabled = true
) {
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectRef = useRef<number | null>(null);
  const token = useAuthStore((s) => s.token);

  const connect = useCallback(() => {
    if (!enabled || !token) return;

    const wsUrl = `ws://localhost:8080/api/v1/ws?token=${encodeURIComponent(token)}`;
    const ws = new WebSocket(wsUrl);

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data) as WebSocketMessage;
        onMessage(data);
      } catch {
        // ignore invalid messages
      }
    };

    ws.onclose = () => {
      wsRef.current = null;
      reconnectRef.current = window.setTimeout(() => {
        connect();
      }, 3000);
    };

    ws.onerror = () => {
      ws.close();
    };

    wsRef.current = ws;
  }, [enabled, token, onMessage]);

  useEffect(() => {
    connect();
    return () => {
      if (reconnectRef.current) {
        clearTimeout(reconnectRef.current);
      }
      wsRef.current?.close();
    };
  }, [connect]);

  const send = useCallback((msg: WebSocketMessage) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
    }
  }, []);

  return { send };
}
