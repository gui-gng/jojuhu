import { useState } from "react";
import { useParams, Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Skeleton } from "@/components/ui/skeleton";
import { useTopic, useTopicReplies, useCreateReply } from "@/hooks/use-api";
import { formatDate } from "@/lib/utils";
import { toast } from "sonner";
import { ArrowLeft, Lock } from "lucide-react";

export function TopicDetailPage() {
  const { forumId, topicId } = useParams();
  const { data: topic, isLoading: topicLoading } = useTopic(forumId || "", topicId || "");
  const { data: replies, isLoading: repliesLoading } = useTopicReplies(forumId || "", topicId || "");
  const createReply = useCreateReply();
  const [content, setContent] = useState("");

  const handleReply = async () => {
    if (!content.trim() || !forumId || !topicId) return;
    try {
      await createReply.mutateAsync({ forumId, topicId, content: content.trim() });
      setContent("");
      toast.success("Reply posted");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  if (topicLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-10 w-20" />
        <Skeleton className="h-40 w-full" />
      </div>
    );
  }

  if (!topic) return <p className="text-muted-foreground">Topic not found.</p>;

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="sm" asChild>
          <Link to={`/forums/${forumId}`}>
            <ArrowLeft className="mr-1 h-4 w-4" /> Back to forum
          </Link>
        </Button>
      </div>

      <div className="rounded-xl border bg-card p-6 shadow-sm">
        <div className="flex items-center gap-2">
          <Avatar className="h-10 w-10">
            <AvatarImage src={topic.user?.avatar_url} />
            <AvatarFallback>{topic.user?.display_name?.[0] || "?"}</AvatarFallback>
          </Avatar>
          <div>
            <p className="font-medium">{topic.user?.display_name}</p>
            <p className="text-xs text-muted-foreground">@{topic.user?.username} · {formatDate(topic.created_at)}</p>
          </div>
        </div>
        <h1 className="mt-4 text-xl font-bold">{topic.title}</h1>
        <p className="mt-2 whitespace-pre-wrap text-sm leading-relaxed">{topic.content}</p>
        {topic.is_locked && (
          <div className="mt-4 flex items-center gap-2 rounded-md bg-muted p-2 text-sm text-muted-foreground">
            <Lock className="h-4 w-4" /> This topic is locked.
          </div>
        )}
      </div>

      {!topic.is_locked && (
        <div className="rounded-xl border bg-card p-4 shadow-sm">
          <textarea
            className="w-full rounded-md border bg-transparent px-3 py-2 text-sm"
            rows={3}
            placeholder="Write a reply..."
            value={content}
            onChange={(e) => setContent(e.target.value)}
          />
          <div className="mt-2 flex justify-end">
            <Button onClick={handleReply} disabled={!content.trim() || createReply.isPending}>
              {createReply.isPending ? "Posting..." : "Reply"}
            </Button>
          </div>
        </div>
      )}

      <div className="space-y-3">
        {repliesLoading ? (
          <>
            <Skeleton className="h-24 w-full" />
            <Skeleton className="h-24 w-full" />
          </>
        ) : replies?.replies?.length ? (
          replies.replies.map((reply) => (
            <div key={reply.id} className="rounded-xl border bg-card p-4">
              <div className="flex items-center gap-2">
                <Avatar className="h-8 w-8">
                  <AvatarImage src={reply.user?.avatar_url} />
                  <AvatarFallback>{reply.user?.display_name?.[0] || "?"}</AvatarFallback>
                </Avatar>
                <span className="font-medium">{reply.user?.display_name}</span>
                <span className="text-sm text-muted-foreground">@{reply.user?.username} · {formatDate(reply.created_at)}</span>
              </div>
              <p className="mt-2 whitespace-pre-wrap text-sm">{reply.content}</p>
            </div>
          ))
        ) : (
          <p className="text-center text-muted-foreground">No replies yet.</p>
        )}
      </div>
    </div>
  );
}
