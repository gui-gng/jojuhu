import { useEffect, useRef, useState } from "react";
import { useParams, Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import {
  useConversations,
  useMessages,
  useSendMessage,
  useMe,
} from "@/hooks/use-api";
import { useWebSocket } from "@/hooks/use-websocket";
import { formatDate } from "@/lib/utils";
import { toast } from "sonner";
import type { WebSocketMessage } from "@/lib/types";
import { useQueryClient } from "@tanstack/react-query";

export function MessagesPage() {
  const { userId } = useParams();
  const { data: conversations, isLoading: convLoading } = useConversations();
  const { data: me } = useMe();
  const qc = useQueryClient();

  useWebSocket((msg: WebSocketMessage) => {
    if (msg.type === "new_message") {
      qc.invalidateQueries({ queryKey: ["conversations"] });
      const payload = msg.payload as { sender_id: string; recipient_id: string } | undefined;
      if (payload) {
        const otherId = payload.sender_id === me?.id ? payload.recipient_id : payload.sender_id;
        qc.invalidateQueries({ queryKey: ["messages", otherId] });
      }
    }
  });

  return (
    <div className="flex h-[calc(100vh-3rem)] gap-4">
      <aside className="flex w-64 flex-col rounded-xl border bg-card">
        <div className="p-4">
          <h2 className="font-semibold">Messages</h2>
        </div>
        <Separator />
        <ScrollArea className="flex-1">
          {convLoading ? (
            <div className="space-y-2 p-3">
              <Skeleton className="h-12 w-full" />
              <Skeleton className="h-12 w-full" />
            </div>
          ) : conversations?.length ? (
            conversations.map((c) => (
              <Link
                key={c.user_id}
                to={`/messages/${c.user_id}`}
                className={`flex items-center gap-3 border-b p-3 transition-colors hover:bg-accent ${
                  userId === c.user_id ? "bg-accent" : ""
                }`}
              >
                <Avatar className="h-10 w-10">
                  <AvatarImage src={c.avatar_url} />
                  <AvatarFallback>{c.display_name?.[0] || "?"}</AvatarFallback>
                </Avatar>
                <div className="min-w-0 flex-1">
                  <p className="truncate text-sm font-medium">{c.display_name}</p>
                  <p className="truncate text-xs text-muted-foreground">{c.last_message}</p>
                </div>
                {c.unread_count > 0 && (
                  <span className="flex h-5 w-5 items-center justify-center rounded-full bg-primary text-[10px] font-bold text-primary-foreground">
                    {c.unread_count}
                  </span>
                )}
              </Link>
            ))
          ) : (
            <p className="p-4 text-sm text-muted-foreground">No conversations yet.</p>
          )}
        </ScrollArea>
      </aside>

      <div className="flex flex-1 flex-col rounded-xl border bg-card">
        {userId ? (
          <ChatThread userId={userId} />
        ) : (
          <div className="flex flex-1 items-center justify-center text-muted-foreground">
            Select a conversation to start messaging
          </div>
        )}
      </div>
    </div>
  );
}

function ChatThread({ userId }: { userId: string }) {
  const { data: messagesData, isLoading } = useMessages(userId);
  const sendMessage = useSendMessage();
  const { data: me } = useMe();
  const [text, setText] = useState("");
  const bottomRef = useRef<HTMLDivElement>(null);
  const qc = useQueryClient();

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messagesData]);

  const handleSend = async () => {
    if (!text.trim()) return;
    try {
      await sendMessage.mutateAsync({ recipient_id: userId, content: text.trim() });
      setText("");
      qc.invalidateQueries({ queryKey: ["messages", userId] });
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <>
      <div className="flex-1 overflow-hidden">
        <ScrollArea className="h-full p-4">
          {isLoading ? (
            <div className="space-y-3">
              <Skeleton className="h-16 w-3/4" />
              <Skeleton className="h-16 w-3/4 self-end" />
            </div>
          ) : (
            <div className="flex flex-col gap-3">
              {messagesData?.map((msg) => {
                const isMe = msg.sender_id === me?.id;
                return (
                  <div
                    key={msg.id}
                    className={`max-w-[70%] rounded-2xl px-4 py-2 text-sm ${
                      isMe
                        ? "self-end bg-primary text-primary-foreground"
                        : "self-start bg-muted"
                    }`}
                  >
                    <p>{msg.content}</p>
                    <span className="mt-1 block text-[10px] opacity-70">
                      {formatDate(msg.created_at)}
                    </span>
                  </div>
                );
              })}
              <div ref={bottomRef} />
            </div>
          )}
        </ScrollArea>
      </div>
      <div className="flex items-center gap-2 border-t p-3">
        <Input
          placeholder="Type a message..."
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && !e.shiftKey) {
              e.preventDefault();
              handleSend();
            }
          }}
        />
        <Button onClick={handleSend} disabled={!text.trim() || sendMessage.isPending}>
          Send
        </Button>
      </div>
    </>
  );
}
