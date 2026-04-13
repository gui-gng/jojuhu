import { Link } from "react-router-dom";
import { Heart, MessageCircle, Repeat2, MoreHorizontal, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import type { Post } from "@/lib/types";
import { formatDate } from "@/lib/utils";
import { useLikePost, useUnlikePost, useRepost, useRemoveRepost, useDeletePost, useMe } from "@/hooks/use-api";
import { toast } from "sonner";

interface PostCardProps {
  post: Post;
  onDelete?: () => void;
}

export function PostCard({ post, onDelete }: PostCardProps) {
  const like = useLikePost();
  const unlike = useUnlikePost();
  const repost = useRepost();
  const removeRepost = useRemoveRepost();
  const deletePost = useDeletePost();
  const { data: me } = useMe();

  const isOwner = me?.id === post.user_id;
  const liked = post.is_liked;
  const reposted = post.is_reposted;

  const handleLike = async () => {
    try {
      if (liked) await unlike.mutateAsync(post.id);
      else await like.mutateAsync(post.id);
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  const handleRepost = async () => {
    try {
      if (reposted) await removeRepost.mutateAsync(post.id);
      else await repost.mutateAsync(post.id);
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  const handleDelete = async () => {
    try {
      await deletePost.mutateAsync(post.id);
      onDelete?.();
      toast.success("Post deleted");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <article className="rounded-xl border bg-card p-4 shadow-sm transition-colors hover:bg-card/60">
      <div className="flex items-start gap-3">
        <Link to={`/u/${post.user?.username || post.user_id}`}>
          <Avatar className="h-10 w-10">
            <AvatarImage src={post.user?.avatar_url} />
            <AvatarFallback>{post.user?.display_name?.[0] || "?"}</AvatarFallback>
          </Avatar>
        </Link>
        <div className="min-w-0 flex-1">
          <div className="flex items-center justify-between gap-2">
            <div className="flex items-center gap-2 overflow-hidden">
              <Link to={`/u/${post.user?.username || post.user_id}`} className="truncate font-semibold hover:underline">
                {post.user?.display_name || "User"}
              </Link>
              <span className="truncate text-sm text-muted-foreground">@{post.user?.username}</span>
              <span className="text-muted-foreground">·</span>
              <span className="shrink-0 text-sm text-muted-foreground">{formatDate(post.created_at)}</span>
            </div>
            {isOwner && (
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="ghost" size="icon" className="h-8 w-8">
                    <MoreHorizontal className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={handleDelete} className="text-destructive">
                    <Trash2 className="mr-2 h-4 w-4" /> Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            )}
          </div>

          <Link to={`/posts/${post.id}`}>
            <p className="mt-1 whitespace-pre-wrap text-sm leading-relaxed">{post.content}</p>
          </Link>

          {post.media_urls && post.media_urls.length > 0 && (
            <div className={`mt-3 grid gap-2 ${post.media_urls.length > 1 ? "grid-cols-2" : "grid-cols-1"}`}>
              {post.media_urls.map((url, i) => (
                <img
                  key={i}
                  src={url}
                  alt=""
                  className="max-h-80 w-full rounded-lg border object-cover"
                />
              ))}
            </div>
          )}

          <div className="mt-3 flex items-center gap-1">
            <Button
              variant="ghost"
              size="sm"
              onClick={handleLike}
              className={`gap-1.5 text-muted-foreground ${liked ? "text-rose-500" : ""}`}
            >
              <Heart className={`h-4 w-4 ${liked ? "fill-current" : ""}`} />
              <span className="text-xs">{post.likes_count || 0}</span>
            </Button>
            <Button variant="ghost" size="sm" className="gap-1.5 text-muted-foreground" asChild>
              <Link to={`/posts/${post.id}`}>
                <MessageCircle className="h-4 w-4" />
                <span className="text-xs">{post.comments_count || 0}</span>
              </Link>
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleRepost}
              className={`gap-1.5 text-muted-foreground ${reposted ? "text-emerald-500" : ""}`}
            >
              <Repeat2 className="h-4 w-4" />
              <span className="text-xs">{post.reposts_count || 0}</span>
            </Button>
          </div>
        </div>
      </div>
    </article>
  );
}
