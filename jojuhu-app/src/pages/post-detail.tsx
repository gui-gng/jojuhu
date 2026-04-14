import { useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Skeleton } from "@/components/ui/skeleton";
import { PostCard } from "@/components/post-card";
import { usePost, useComments, useCreateComment, useMe } from "@/hooks/use-api";
import { formatDate } from "@/lib/utils";
import { toast } from "sonner";
import { ArrowLeft } from "lucide-react";

export function PostDetailPage() {
  const { postId } = useParams();
  const navigate = useNavigate();
  const { data: post, isLoading: postLoading } = usePost(postId || "");
  const { data: commentsData, isLoading: commentsLoading } = useComments(postId || "");
  const { data: me } = useMe();
  const createComment = useCreateComment();
  const [content, setContent] = useState("");

  const handleComment = async () => {
    if (!content.trim() || !postId) return;
    try {
      await createComment.mutateAsync({ postId, content: content.trim() });
      setContent("");
      toast.success("Comment posted");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  if (postLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-10 w-20" />
        <Skeleton className="h-40 w-full" />
      </div>
    );
  }

  if (!post) return <p className="text-muted-foreground">Post not found.</p>;

  return (
    <div className="space-y-6">
      <Button variant="ghost" onClick={() => navigate(-1)} className="gap-2">
        <ArrowLeft className="h-4 w-4" /> Back
      </Button>

      <PostCard post={post} />

      <div className="rounded-xl border bg-card p-4 shadow-sm">
        <div className="flex gap-3">
          <Avatar className="h-10 w-10">
            <AvatarImage src={me?.avatar_url} />
            <AvatarFallback>{me?.display_name?.[0] || "?"}</AvatarFallback>
          </Avatar>
          <div className="flex-1 space-y-2">
            <Textarea
              placeholder="Write a comment..."
              value={content}
              onChange={(e) => setContent(e.target.value)}
              rows={2}
              className="resize-none"
            />
            <div className="flex justify-end">
              <Button onClick={handleComment} disabled={!content.trim() || createComment.isPending}>
                {createComment.isPending ? "Posting..." : "Comment"}
              </Button>
            </div>
          </div>
        </div>
      </div>

      <div className="space-y-4">
        {commentsLoading ? (
          <>
            <Skeleton className="h-24 w-full" />
            <Skeleton className="h-24 w-full" />
          </>
        ) : commentsData?.length ? (
          commentsData.map((comment) => (
            <div key={comment.id} className="rounded-xl border bg-card p-4">
              <div className="flex items-center gap-2">
                <Avatar className="h-8 w-8">
                  <AvatarImage src={comment.user?.avatar_url} />
                  <AvatarFallback>{comment.user?.display_name?.[0] || "?"}</AvatarFallback>
                </Avatar>
                <span className="font-medium">{comment.user?.display_name}</span>
                <span className="text-sm text-muted-foreground">@{comment.user?.username}</span>
                <span className="text-muted-foreground">·</span>
                <span className="text-sm text-muted-foreground">{formatDate(comment.created_at)}</span>
              </div>
              <p className="mt-2 whitespace-pre-wrap text-sm">{comment.content}</p>
            </div>
          ))
        ) : (
          <p className="text-center text-muted-foreground">No comments yet. Be the first!</p>
        )}
      </div>
    </div>
  );
}
