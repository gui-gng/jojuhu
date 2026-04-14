import { useState } from "react";
import { useParams } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Skeleton } from "@/components/ui/skeleton";
import { PostCard } from "@/components/post-card";
import { useUser, useMe, useFollow, useUnfollow, useUserPosts, useUpdateProfile } from "@/hooks/use-api";
import { formatNumber } from "@/lib/utils";
import { toast } from "sonner";
import { Calendar, Check, UserPlus, UserMinus } from "lucide-react";

export function ProfilePage() {
  const { username } = useParams();
  const { data: me } = useMe();
  const isMe = !username || username === me?.username;
  const { data: profile, isLoading } = useUser(isMe ? me?.id || "" : "");
  const { data: posts } = useUserPosts(isMe ? me?.id || "" : profile?.id || "", 1);
  const updateProfile = useUpdateProfile();
  const [editing, setEditing] = useState(false);
  const [bio, setBio] = useState(me?.bio || "");
  const [displayName, setDisplayName] = useState(me?.display_name || "");

  // If viewing another user by username, we'd need a lookup-by-username endpoint.
  // For now, render self profile if no username or match.
  const user = isMe ? me : profile;

  const handleSave = async () => {
    try {
      await updateProfile.mutateAsync({ bio, display_name: displayName });
      setEditing(false);
      toast.success("Profile updated");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  if (isLoading || !user) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-32 w-full" />
        <Skeleton className="h-8 w-48" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="rounded-xl border bg-card p-6 shadow-sm">
        <div className="flex items-start justify-between gap-4">
          <Avatar className="h-24 w-24">
            <AvatarImage src={user.avatar_url} />
            <AvatarFallback className="text-2xl">{user.display_name?.[0] || user.username[0]}</AvatarFallback>
          </Avatar>
          <div className="flex-1">
            {editing ? (
              <div className="space-y-2">
                <input
                  className="w-full rounded-md border px-2 py-1 text-xl font-bold"
                  value={displayName}
                  onChange={(e) => setDisplayName(e.target.value)}
                />
                <textarea
                  className="w-full rounded-md border px-2 py-1 text-sm"
                  rows={2}
                  value={bio}
                  onChange={(e) => setBio(e.target.value)}
                />
                <div className="flex gap-2">
                  <Button size="sm" onClick={handleSave}>Save</Button>
                  <Button size="sm" variant="secondary" onClick={() => setEditing(false)}>Cancel</Button>
                </div>
              </div>
            ) : (
              <>
                <h1 className="text-2xl font-bold">{user.display_name}</h1>
                <p className="text-muted-foreground">@{user.username}</p>
                {user.is_verified && (
                  <span className="inline-flex items-center gap-1 text-xs text-primary">
                    <Check className="h-3 w-3" /> Verified
                  </span>
                )}
                <p className="mt-2 text-sm">{user.bio || "No bio yet."}</p>
                <div className="mt-3 flex flex-wrap gap-4 text-sm text-muted-foreground">
                  <span className="flex items-center gap-1">
                    <Calendar className="h-4 w-4" /> Joined {new Date(user.created_at).toLocaleDateString()}
                  </span>
                </div>
              </>
            )}
          </div>
          <div>
            {isMe ? (
              <Button variant="outline" onClick={() => setEditing(true)}>Edit profile</Button>
            ) : (
              <FollowButton userId={user.id} />
            )}
          </div>
        </div>

        <div className="mt-6 flex gap-6 text-sm">
          <div>
            <span className="font-semibold">{formatNumber(user.posts_count)}</span>{" "}
            <span className="text-muted-foreground">Posts</span>
          </div>
          <div>
            <span className="font-semibold">{formatNumber(user.followers_count)}</span>{" "}
            <span className="text-muted-foreground">Followers</span>
          </div>
          <div>
            <span className="font-semibold">{formatNumber(user.following_count)}</span>{" "}
            <span className="text-muted-foreground">Following</span>
          </div>
        </div>
      </div>

      <Tabs defaultValue="posts">
        <TabsList className="w-full">
          <TabsTrigger value="posts">Posts</TabsTrigger>
          <TabsTrigger value="media">Media</TabsTrigger>
          <TabsTrigger value="likes">Likes</TabsTrigger>
        </TabsList>
        <TabsContent value="posts" className="space-y-4">
          {posts?.length ? (
            posts.map((post) => <PostCard key={post.id} post={post} />)
          ) : (
            <p className="text-center text-muted-foreground">No posts yet.</p>
          )}
        </TabsContent>
        <TabsContent value="media">
          <p className="text-center text-muted-foreground">Media gallery coming soon.</p>
        </TabsContent>
        <TabsContent value="likes">
          <p className="text-center text-muted-foreground">Liked posts coming soon.</p>
        </TabsContent>
      </Tabs>
    </div>
  );
}

function FollowButton({ userId }: { userId: string }) {
  const follow = useFollow();
  const unfollow = useUnfollow();
  const [following, setFollowing] = useState(false);

  const handleFollow = async () => {
    try {
      await follow.mutateAsync(userId);
      setFollowing(true);
    } catch {}
  };

  const handleUnfollow = async () => {
    try {
      await unfollow.mutateAsync(userId);
      setFollowing(false);
    } catch {}
  };

  return following ? (
    <Button variant="outline" onClick={handleUnfollow}>
      <UserMinus className="mr-2 h-4 w-4" /> Unfollow
    </Button>
  ) : (
    <Button onClick={handleFollow}>
      <UserPlus className="mr-2 h-4 w-4" /> Follow
    </Button>
  );
}
