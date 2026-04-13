import { useState } from "react";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { PostCard } from "@/components/post-card";
import { useFeed, useCreatePost, useMe } from "@/hooks/use-api";
import { Skeleton } from "@/components/ui/skeleton";
import { toast } from "sonner";

export function FeedPage() {
  const [tab, setTab] = useState<"feed" | "following" | "for-you" | "trending">("feed");
  const { data, isLoading, refetch } = useFeed(tab);
  const { data: me } = useMe();
  const createPost = useCreatePost();
  const [content, setContent] = useState("");

  const handlePost = async () => {
    if (!content.trim()) return;
    try {
      await createPost.mutateAsync({ content: content.trim(), is_public: true });
      setContent("");
      refetch();
      toast.success("Posted!");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <div className="space-y-6">
      <Tabs value={tab} onValueChange={(v) => setTab(v as typeof tab)}>
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="feed">Feed</TabsTrigger>
          <TabsTrigger value="following">Following</TabsTrigger>
          <TabsTrigger value="for-you">For You</TabsTrigger>
          <TabsTrigger value="trending">Trending</TabsTrigger>
        </TabsList>
      </Tabs>

      <div className="rounded-xl border bg-card p-4 shadow-sm">
        <div className="flex gap-3">
          <Avatar className="h-10 w-10">
            <AvatarImage src={me?.avatar_url} />
            <AvatarFallback>{me?.display_name?.[0] || "?"}</AvatarFallback>
          </Avatar>
          <div className="flex-1 space-y-2">
            <Textarea
              placeholder="What's happening?"
              value={content}
              onChange={(e) => setContent(e.target.value)}
              rows={2}
              className="resize-none"
            />
            <div className="flex justify-end">
              <Button onClick={handlePost} disabled={!content.trim() || createPost.isPending}>
                {createPost.isPending ? "Posting..." : "Post"}
              </Button>
            </div>
          </div>
        </div>
      </div>

      <div className="space-y-4">
        {isLoading ? (
          <>
            <Skeleton className="h-32 w-full" />
            <Skeleton className="h-32 w-full" />
            <Skeleton className="h-32 w-full" />
          </>
        ) : data?.posts?.length ? (
          data.posts.map((post) => <PostCard key={post.id} post={post} onDelete={() => refetch()} />)
        ) : (
          <p className="text-center text-muted-foreground">No posts to show.</p>
        )}
      </div>
    </div>
  );
}
