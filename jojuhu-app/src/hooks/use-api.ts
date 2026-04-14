import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import type {
  User, LoginResponse, Post, Comment, Forum, Topic, Reply,
  Message, Conversation, Notification, Story, Group, SearchResult,
  Hashtag, PrivacySettings, Poll, GroupMember
} from "@/lib/types";

const STALE_TIME = 1000 * 60 * 5;

// Auth
export const useMe = () =>
  useQuery({
    queryKey: ["me"],
    queryFn: () => api.get<User>("/api/v1/users/me"),
    staleTime: STALE_TIME,
  });

export const useLogin = () =>
  useMutation({
    mutationFn: (body: { username_or_email: string; password: string }) =>
      api.post<LoginResponse>("/api/v1/auth/login", body),
  });

export const useRegister = () =>
  useMutation({
    mutationFn: (body: { username: string; email: string; password: string; display_name: string }) =>
      api.post<LoginResponse>("/api/v1/auth/register", body),
  });

// Users
export const useUser = (id: string) =>
  useQuery({
    queryKey: ["users", id],
    queryFn: () => api.get<User>(`/api/v1/users/${id}`),
    enabled: !!id,
    staleTime: STALE_TIME,
  });

export const useFollow = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/users/${id}/follow`),
    onSuccess: (_, id) => {
      qc.invalidateQueries({ queryKey: ["users", id] });
      qc.invalidateQueries({ queryKey: ["me"] });
    },
  });
};

export const useUnfollow = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.delete<void>(`/api/v1/users/${id}/follow`),
    onSuccess: (_, id) => {
      qc.invalidateQueries({ queryKey: ["users", id] });
      qc.invalidateQueries({ queryKey: ["me"] });
    },
  });
};

export const useUpdateProfile = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: Partial<User>) => api.put<User>("/api/v1/users/me", body),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["me"] }),
  });
};

// Timeline / Posts
export const useFeed = (type: "feed" | "following" | "for-you" | "trending", page = 1) =>
  useQuery({
    queryKey: ["timeline", type, page],
    queryFn: () =>
      api.get<Post[]>(`/api/v1/timeline/${type}?page=${page}&per_page=20`),
    staleTime: 1000 * 30,
  });

export const useUserPosts = (userId: string, page = 1) =>
  useQuery({
    queryKey: ["posts", "user", userId, page],
    queryFn: () => api.get<Post[]>(`/api/v1/timeline/users/${userId}/posts?page=${page}&per_page=20`),
    enabled: !!userId,
  });

export const usePost = (id: string) =>
  useQuery({
    queryKey: ["posts", id],
    queryFn: () => api.get<Post>(`/api/v1/timeline/posts/${id}`),
    enabled: !!id,
  });

export const useCreatePost = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: { content: string; is_public?: boolean; media_urls?: string[] }) =>
      api.post<Post>("/api/v1/timeline/posts", body),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["timeline"] });
    },
  });
};

export const useDeletePost = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.delete<void>(`/api/v1/timeline/posts/${id}`),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["timeline"] }),
  });
};

export const useLikePost = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/timeline/posts/${id}/like`),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["timeline"] });
      qc.invalidateQueries({ queryKey: ["posts"] });
    },
  });
};

export const useUnlikePost = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.delete<void>(`/api/v1/timeline/posts/${id}/like`),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["timeline"] });
      qc.invalidateQueries({ queryKey: ["posts"] });
    },
  });
};

export const useComments = (postId: string) =>
  useQuery({
    queryKey: ["comments", postId],
    queryFn: () => api.get<Comment[]>(`/api/v1/timeline/posts/${postId}/comments`),
    enabled: !!postId,
  });

export const useCreateComment = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ postId, content }: { postId: string; content: string }) =>
      api.post<Comment>(`/api/v1/timeline/posts/${postId}/comments`, { content }),
    onSuccess: (_, { postId }) => {
      qc.invalidateQueries({ queryKey: ["comments", postId] });
      qc.invalidateQueries({ queryKey: ["posts", postId] });
      qc.invalidateQueries({ queryKey: ["timeline"] });
    },
  });
};

export const useRepost = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (postId: string) => api.post<void>("/api/v1/timeline/reposts", { post_id: postId }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["timeline"] }),
  });
};

export const useRemoveRepost = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (postId: string) => api.delete<void>(`/api/v1/timeline/reposts/${postId}`),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["timeline"] }),
  });
};

// Forums
export const useForums = (page = 1) =>
  useQuery({
    queryKey: ["forums", page],
    queryFn: () => api.get<Forum[]>(`/api/v1/forums?page=${page}&per_page=20`),
  });

export const useForum = (id: string) =>
  useQuery({
    queryKey: ["forums", id],
    queryFn: () => api.get<Forum>(`/api/v1/forums/${id}`),
    enabled: !!id,
  });

export const useForumTopics = (forumId: string, page = 1) =>
  useQuery({
    queryKey: ["forums", forumId, "topics", page],
    queryFn: () => api.get<Topic[]>(`/api/v1/forums/${forumId}/topics?page=${page}&per_page=20`),
    enabled: !!forumId,
  });

export const useTopic = (forumId: string, topicId: string) =>
  useQuery({
    queryKey: ["forums", forumId, "topics", topicId],
    queryFn: () => api.get<Topic>(`/api/v1/forums/${forumId}/topics/${topicId}`),
    enabled: !!forumId && !!topicId,
  });

export const useTopicReplies = (forumId: string, topicId: string, page = 1) =>
  useQuery({
    queryKey: ["forums", forumId, "topics", topicId, "replies", page],
    queryFn: () =>
      api.get<Reply[]>(`/api/v1/forums/${forumId}/topics/${topicId}/replies?page=${page}&per_page=20`),
    enabled: !!forumId && !!topicId,
  });

export const useCreateForum = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: { name: string; description: string; is_public?: boolean }) =>
      api.post<Forum>("/api/v1/forums", body),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["forums"] }),
  });
};

export const useJoinForum = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/forums/${id}/join`),
    onSuccess: (_, id) => qc.invalidateQueries({ queryKey: ["forums", id] }),
  });
};

export const useLeaveForum = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/forums/${id}/leave`),
    onSuccess: (_, id) => qc.invalidateQueries({ queryKey: ["forums", id] }),
  });
};

export const useCreateTopic = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ forumId, title, content }: { forumId: string; title: string; content: string }) =>
      api.post<Topic>(`/api/v1/forums/${forumId}/topics`, { title, content }),
    onSuccess: (_, { forumId }) => qc.invalidateQueries({ queryKey: ["forums", forumId, "topics"] }),
  });
};

export const useCreateReply = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({
      forumId,
      topicId,
      content,
    }: {
      forumId: string;
      topicId: string;
      content: string;
    }) => api.post<Reply>(`/api/v1/forums/${forumId}/topics/${topicId}/replies`, { content }),
    onSuccess: (_, { forumId, topicId }) => {
      qc.invalidateQueries({ queryKey: ["forums", forumId, "topics", topicId, "replies"] });
      qc.invalidateQueries({ queryKey: ["forums", forumId, "topics"] });
    },
  });
};

// Messages
export const useConversations = () =>
  useQuery({
    queryKey: ["conversations"],
    queryFn: () => api.get<Conversation[]>("/api/v1/messages/conversations"),
  });

export const useMessages = (userId: string, page = 1) =>
  useQuery({
    queryKey: ["messages", userId, page],
    queryFn: () =>
      api.get<Message[]>(`/api/v1/messages/conversations/${userId}?page=${page}&per_page=50`),
    enabled: !!userId,
  });

export const useSendMessage = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: { recipient_id: string; content: string }) =>
      api.post<Message>("/api/v1/messages", body),
    onSuccess: (_, { recipient_id }) => {
      qc.invalidateQueries({ queryKey: ["messages", recipient_id] });
      qc.invalidateQueries({ queryKey: ["conversations"] });
    },
  });
};

export const useMarkRead = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (messageId: string) => api.put<void>(`/api/v1/messages/${messageId}/read`),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["conversations"] }),
  });
};

// Notifications
export const useNotifications = (unreadOnly = false) =>
  useQuery({
    queryKey: ["notifications", unreadOnly],
    queryFn: () =>
      api.get<Notification[]>(`/api/v1/notifications${unreadOnly ? "?unread_only=true" : ""}`),
  });

export const useNotificationCount = () =>
  useQuery({
    queryKey: ["notifications", "count"],
    queryFn: () => api.get<{ count: number }>("/api/v1/notifications/count"),
  });

export const useMarkNotificationRead = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/notifications/${id}/read`),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["notifications"] });
      qc.invalidateQueries({ queryKey: ["notifications", "count"] });
    },
  });
};

export const useMarkAllNotificationsRead = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: () => api.post<void>("/api/v1/notifications/read-all"),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["notifications"] });
      qc.invalidateQueries({ queryKey: ["notifications", "count"] });
    },
  });
};

// Stories
export const useStoriesFeed = () =>
  useQuery({
    queryKey: ["stories", "feed"],
    queryFn: () => api.get<Story[]>("/api/v1/stories/feed"),
  });

export const useFollowingStories = () =>
  useQuery({
    queryKey: ["stories", "following"],
    queryFn: () => api.get<Story[]>("/api/v1/stories/following"),
  });

export const useMyStories = () =>
  useQuery({
    queryKey: ["stories", "me"],
    queryFn: () => api.get<Story[]>("/api/v1/stories/me"),
  });

export const useCreateStory = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: { media_url: string; media_type: "image" | "video"; caption?: string }) =>
      api.post<Story>("/api/v1/stories", body),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["stories"] });
    },
  });
};

export const useViewStory = () =>
  useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/stories/${id}/view`),
  });

// Groups
export const useGroups = (page = 1) =>
  useQuery({
    queryKey: ["groups", page],
    queryFn: () => api.get<Group[]>(`/api/v1/groups?page=${page}&per_page=20`),
  });

export const useGroup = (id: string) =>
  useQuery({
    queryKey: ["groups", id],
    queryFn: () => api.get<Group>(`/api/v1/groups/${id}`),
    enabled: !!id,
  });

export const useGroupMembers = (id: string) =>
  useQuery({
    queryKey: ["groups", id, "members"],
    queryFn: () => api.get<GroupMember[]>(`/api/v1/groups/${id}/members`),
    enabled: !!id,
  });

export const useCreateGroup = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: { name: string; description: string; is_public?: boolean }) =>
      api.post<Group>("/api/v1/groups", body),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["groups"] }),
  });
};

export const useJoinGroup = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/groups/${id}/join`),
    onSuccess: (_, id) => qc.invalidateQueries({ queryKey: ["groups", id] }),
  });
};

export const useLeaveGroup = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.post<void>(`/api/v1/groups/${id}/leave`),
    onSuccess: (_, id) => qc.invalidateQueries({ queryKey: ["groups", id] }),
  });
};

// Search
export const useSearch = (q: string, type = "all") =>
  useQuery({
    queryKey: ["search", q, type],
    queryFn: () =>
      api.get<SearchResult>(`/api/v1/search?q=${encodeURIComponent(q)}&search_type=${type}`),
    enabled: q.length > 1,
  });

export const useTrendingHashtags = () =>
  useQuery({
    queryKey: ["hashtags", "trending"],
    queryFn: () => api.get<Hashtag[]>("/api/v1/hashtag/trending"),
  });

// Privacy
export const usePrivacySettings = () =>
  useQuery({
    queryKey: ["privacy"],
    queryFn: () => api.get<PrivacySettings>("/api/v1/privacy/settings"),
  });

export const useUpdatePrivacy = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: Partial<PrivacySettings>) =>
      api.put<PrivacySettings>("/api/v1/privacy/settings", body),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["privacy"] }),
  });
};

// Uploads
export const usePresignedUrl = () =>
  useMutation({
    mutationFn: (body: { filename: string; content_type: string }) =>
      api.post<{ url: string; key: string }>("/api/v1/upload/presigned-url", body),
  });

// Polls
export const useCreatePoll = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ postId, question, options }: { postId: string; question: string; options: string[] }) =>
      api.post<Poll>(`/api/v1/polls/${postId}`, { question, options }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["timeline"] }),
  });
};

export const useVotePoll = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ postId, optionId }: { postId: string; optionId: string }) =>
      api.post<void>(`/api/v1/polls/${postId}/vote`, { option_id: optionId }),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["timeline"] });
      qc.invalidateQueries({ queryKey: ["posts"] });
    },
  });
};
