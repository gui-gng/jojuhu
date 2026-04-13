
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Skeleton } from "@/components/ui/skeleton";
import { useNotifications, useMarkNotificationRead, useMarkAllNotificationsRead } from "@/hooks/use-api";
import { formatDate } from "@/lib/utils";
import { toast } from "sonner";
import { Heart, MessageCircle, UserPlus, Users, AtSign, Bell } from "lucide-react";

const iconMap: Record<string, React.ElementType> = {
  PostLike: Heart,
  PostComment: MessageCircle,
  NewFollower: UserPlus,
  ForumReply: Users,
  Mention: AtSign,
  default: Bell,
};

export function NotificationsPage() {
  const { data, isLoading, refetch } = useNotifications();
  const markRead = useMarkNotificationRead();
  const markAll = useMarkAllNotificationsRead();

  const handleMarkAll = async () => {
    try {
      await markAll.mutateAsync();
      refetch();
      toast.success("All marked as read");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Notifications</h1>
        <Button variant="outline" size="sm" onClick={handleMarkAll}>
          Mark all as read
        </Button>
      </div>

      <div className="space-y-2">
        {isLoading ? (
          <>
            <Skeleton className="h-20 w-full" />
            <Skeleton className="h-20 w-full" />
            <Skeleton className="h-20 w-full" />
          </>
        ) : data?.notifications?.length ? (
          data.notifications.map((n) => {
            const Icon = iconMap[n.type] || iconMap.default;
            return (
              <div
                key={n.id}
                className={`flex items-start gap-4 rounded-xl border p-4 transition-colors hover:bg-accent ${
                  !n.is_read ? "bg-accent/30" : "bg-card"
                }`}
              >
                <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-primary/10">
                  <Icon className="h-5 w-5 text-primary" />
                </div>
                <div className="min-w-0 flex-1">
                  <div className="flex items-center gap-2">
                    <Avatar className="h-6 w-6">
                      <AvatarImage src={n.actor?.avatar_url} />
                      <AvatarFallback>{n.actor?.display_name?.[0] || "?"}</AvatarFallback>
                    </Avatar>
                    <span className="font-medium">{n.actor?.display_name || "Someone"}</span>
                    <span className="text-sm text-muted-foreground">{n.title}</span>
                  </div>
                  <p className="mt-1 text-sm">{n.message}</p>
                  <p className="mt-1 text-xs text-muted-foreground">{formatDate(n.created_at)}</p>
                </div>
                {!n.is_read && (
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={async () => {
                      await markRead.mutateAsync(n.id);
                      refetch();
                    }}
                  >
                    Mark read
                  </Button>
                )}
              </div>
            );
          })
        ) : (
          <p className="text-center text-muted-foreground">No notifications yet.</p>
        )}
      </div>
    </div>
  );
}
