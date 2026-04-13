
import { Routes, Route, Navigate, useLocation } from "react-router-dom";
import { useAuthStore } from "@/store/auth";
import { AppLayout } from "@/components/layout/app-layout";
import { LoginPage } from "@/pages/login";
import { RegisterPage } from "@/pages/register";
import { FeedPage } from "@/pages/feed";
import { PostDetailPage } from "@/pages/post-detail";
import { ProfilePage } from "@/pages/profile";
import { MessagesPage } from "@/pages/messages";
import { ForumsPage } from "@/pages/forums";
import { ForumDetailPage } from "@/pages/forum-detail";
import { TopicDetailPage } from "@/pages/topic-detail";
import { NotificationsPage } from "@/pages/notifications";
import { SearchPage } from "@/pages/search";
import { GroupsPage } from "@/pages/groups";
import { GroupDetailPage } from "@/pages/group-detail";
import { StoriesPage } from "@/pages/stories";
import { SettingsPage } from "@/pages/settings";

function RequireAuth({ children }: { children: React.ReactNode }) {
  const token = useAuthStore((s) => s.token);
  const isInitialized = useAuthStore((s) => s.isInitialized);
  const location = useLocation();

  if (!isInitialized) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent" />
      </div>
    );
  }

  if (!token) {
    return <Navigate to="/login" state={{ from: location }} replace />;
  }

  return <>{children}</>;
}

function RedirectIfAuth({ children }: { children: React.ReactNode }) {
  const token = useAuthStore((s) => s.token);
  const isInitialized = useAuthStore((s) => s.isInitialized);
  if (!isInitialized) {
    return (
      <div className="flex h-full w-full items-center justify-center">
        <div className="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent" />
      </div>
    );
  }
  if (token) return <Navigate to="/" replace />;
  return <>{children}</>;
}

export default function App() {
  return (
    <Routes>
      <Route
        path="/login"
        element={
          <RedirectIfAuth>
            <LoginPage />
          </RedirectIfAuth>
        }
      />
      <Route
        path="/register"
        element={
          <RedirectIfAuth>
            <RegisterPage />
          </RedirectIfAuth>
        }
      />
      <Route
        path="/"
        element={
          <RequireAuth>
            <AppLayout />
          </RequireAuth>
        }
      >
        <Route index element={<FeedPage />} />
        <Route path="posts/:postId" element={<PostDetailPage />} />
        <Route path="search" element={<SearchPage />} />
        <Route path="messages" element={<MessagesPage />} />
        <Route path="messages/:userId" element={<MessagesPage />} />
        <Route path="forums" element={<ForumsPage />} />
        <Route path="forums/:forumId" element={<ForumDetailPage />} />
        <Route path="forums/:forumId/topics/:topicId" element={<TopicDetailPage />} />
        <Route path="groups" element={<GroupsPage />} />
        <Route path="groups/:groupId" element={<GroupDetailPage />} />
        <Route path="notifications" element={<NotificationsPage />} />
        <Route path="stories" element={<StoriesPage />} />
        <Route path="settings" element={<SettingsPage />} />
        <Route path="u/:username" element={<ProfilePage />} />
        <Route path="profile" element={<ProfilePage />} />
      </Route>
    </Routes>
  );
}
