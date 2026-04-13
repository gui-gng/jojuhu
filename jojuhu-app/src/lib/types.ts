export interface User {
  id: string;
  username: string;
  email: string;
  display_name: string;
  bio?: string;
  avatar_url?: string;
  is_verified: boolean;
  is_private: boolean;
  followers_count: number;
  following_count: number;
  posts_count: number;
  created_at: string;
  updated_at: string;
}

export interface LoginResponse {
  token: string;
  user_id: string;
  user: User;
}

export interface Post {
  id: string;
  user_id: string;
  content: string;
  media_urls: string[];
  is_public: boolean;
  likes_count: number;
  comments_count: number;
  reposts_count: number;
  is_liked?: boolean;
  is_reposted?: boolean;
  created_at: string;
  updated_at: string;
  user?: User;
  poll?: Poll;
}

export interface Comment {
  id: string;
  post_id: string;
  user_id: string;
  content: string;
  parent_comment_id?: string;
  created_at: string;
  updated_at: string;
  user?: User;
  replies?: Comment[];
}

export interface Poll {
  id: string;
  post_id: string;
  question: string;
  options: PollOption[];
  total_votes: number;
  has_voted?: boolean;
  ends_at?: string;
}

export interface PollOption {
  id: string;
  text: string;
  vote_count: number;
}

export interface Forum {
  id: string;
  name: string;
  description: string;
  is_public: boolean;
  member_count: number;
  topic_count: number;
  created_by: string;
  created_at: string;
  is_member?: boolean;
}

export interface Topic {
  id: string;
  forum_id: string;
  user_id: string;
  title: string;
  content: string;
  is_locked: boolean;
  is_pinned: boolean;
  reply_count: number;
  created_at: string;
  updated_at: string;
  user?: User;
}

export interface Reply {
  id: string;
  topic_id: string;
  user_id: string;
  content: string;
  parent_reply_id?: string;
  created_at: string;
  updated_at: string;
  user?: User;
}

export interface Message {
  id: string;
  sender_id: string;
  recipient_id: string;
  content: string;
  is_read: boolean;
  created_at: string;
  updated_at: string;
}

export interface Conversation {
  user_id: string;
  username: string;
  display_name: string;
  avatar_url?: string;
  last_message: string;
  last_message_at: string;
  unread_count: number;
}

export interface Notification {
  id: string;
  user_id: string;
  type: string;
  title: string;
  message: string;
  is_read: boolean;
  created_at: string;
  actor?: User;
  reference_id?: string;
}

export interface Story {
  id: string;
  user_id: string;
  media_url: string;
  media_type: "image" | "video";
  caption?: string;
  view_count: number;
  expires_at: string;
  created_at: string;
  user?: User;
}

export interface Group {
  id: string;
  name: string;
  description: string;
  is_public: boolean;
  member_count: number;
  created_by: string;
  created_at: string;
  is_member?: boolean;
  my_role?: "admin" | "moderator" | "member";
}

export interface GroupMember {
  user_id: string;
  username: string;
  display_name: string;
  avatar_url?: string;
  role: "admin" | "moderator" | "member";
  joined_at: string;
}

export interface GroupInvitation {
  id: string;
  group_id: string;
  group_name: string;
  inviter_id: string;
  inviter_name: string;
  created_at: string;
}

export interface GroupJoinRequest {
  id: string;
  group_id: string;
  user_id: string;
  username: string;
  display_name: string;
  created_at: string;
}

export interface SearchResult {
  posts?: Post[];
  users?: User[];
  forums?: Forum[];
  topics?: Topic[];
}

export interface Hashtag {
  name: string;
  usage_count: number;
  posts_count: number;
}

export interface PrivacySettings {
  is_profile_private: boolean;
  allow_dm_from_followers_only: boolean;
  allow_dm_from_anyone: boolean;
  show_activity_status: boolean;
}

export interface WebSocketMessage {
  type:
    | "notification"
    | "new_message"
    | "typing"
    | "user_status"
    | "new_comment"
    | "new_post"
    | "ping"
    | "pong";
  payload?: unknown;
}
