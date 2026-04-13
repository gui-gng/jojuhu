import { Link, Outlet, useLocation, useNavigate } from "react-router-dom";
import {
  Home,
  Search,
  MessageCircle,
  Users,
  Bell,
  Settings,
  Camera,
  LogOut,
  Hash,
  User,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { useAuthStore } from "@/store/auth";
import { useMe, useNotificationCount } from "@/hooks/use-api";
import { cn } from "@/lib/utils";

const navItems = [
  { icon: Home, label: "Feed", path: "/" },
  { icon: Search, label: "Search", path: "/search" },
  { icon: MessageCircle, label: "Messages", path: "/messages" },
  { icon: Users, label: "Forums", path: "/forums" },
  { icon: Hash, label: "Groups", path: "/groups" },
  { icon: Camera, label: "Stories", path: "/stories" },
  { icon: Bell, label: "Notifications", path: "/notifications", badge: true },
  { icon: User, label: "Profile", path: "/profile" },
  { icon: Settings, label: "Settings", path: "/settings" },
];

export function AppLayout() {
  const location = useLocation();
  const navigate = useNavigate();
  const logout = useAuthStore((s) => s.logout);
  const { data: me } = useMe();
  const { data: notifCount } = useNotificationCount();

  const handleLogout = async () => {
    await logout();
    navigate("/login");
  };

  return (
    <div className="flex h-full w-full bg-background">
      {/* Sidebar */}
      <aside className="flex w-64 flex-col border-r bg-card">
        <div className="flex h-14 items-center px-4">
          <Link to="/" className="text-xl font-bold tracking-tight">
            jojuhu
          </Link>
        </div>
        <ScrollArea className="flex-1 px-3 py-2">
          <nav className="flex flex-col gap-1">
            {navItems.map((item) => {
              const active = location.pathname === item.path || location.pathname.startsWith(item.path + "/");
              return (
                <Link
                  key={item.path}
                  to={item.path}
                  className={cn(
                    "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                    active
                      ? "bg-primary text-primary-foreground"
                      : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
                  )}
                >
                  <item.icon className="h-4 w-4" />
                  {item.label}
                  {item.badge && notifCount?.count ? (
                    <span className="ml-auto flex h-5 min-w-[1.25rem] items-center justify-center rounded-full bg-destructive text-[10px] font-bold text-destructive-foreground px-1">
                      {notifCount.count > 99 ? "99+" : notifCount.count}
                    </span>
                  ) : null}
                </Link>
              );
            })}
          </nav>
        </ScrollArea>
        <Separator />
        <div className="flex items-center gap-3 p-3">
          <Avatar className="h-9 w-9">
            <AvatarImage src={me?.avatar_url} />
            <AvatarFallback>{me?.display_name?.[0] || me?.username?.[0] || "?"}</AvatarFallback>
          </Avatar>
          <div className="flex-1 overflow-hidden">
            <p className="truncate text-sm font-medium">{me?.display_name || me?.username}</p>
            <p className="truncate text-xs text-muted-foreground">@{me?.username}</p>
          </div>
          <Button variant="ghost" size="icon" onClick={handleLogout} title="Logout">
            <LogOut className="h-4 w-4" />
          </Button>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-hidden">
        <ScrollArea className="h-full">
          <div className="mx-auto max-w-3xl p-6"><Outlet /></div>
        </ScrollArea>
      </main>
    </div>
  );
}
