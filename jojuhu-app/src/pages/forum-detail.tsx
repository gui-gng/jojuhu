import { useState } from "react";
import { useParams, Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { useForum, useForumTopics, useCreateTopic, useJoinForum, useLeaveForum } from "@/hooks/use-api";
import { Skeleton } from "@/components/ui/skeleton";
import { formatDate, formatNumber } from "@/lib/utils";
import { ArrowLeft, Plus, MessageSquare, Lock, Globe } from "lucide-react";
import { toast } from "sonner";

export function ForumDetailPage() {
  const { forumId } = useParams();
  const { data: forum, isLoading: forumLoading } = useForum(forumId || "");
  const { data: topics, isLoading: topicsLoading } = useForumTopics(forumId || "");
  const createTopic = useCreateTopic();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState({ title: "", content: "" });

  const handleCreate = async () => {
    if (!forumId) return;
    try {
      await createTopic.mutateAsync({ forumId, title: form.title, content: form.content });
      setOpen(false);
      setForm({ title: "", content: "" });
      toast.success("Topic created");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  if (forumLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-10 w-20" />
        <Skeleton className="h-32 w-full" />
      </div>
    );
  }

  if (!forum) return <p className="text-muted-foreground">Forum not found.</p>;

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="sm" asChild>
          <Link to="/forums">
            <ArrowLeft className="mr-1 h-4 w-4" /> Back
          </Link>
        </Button>
      </div>

      <div className="rounded-xl border bg-card p-6 shadow-sm">
        <div className="flex items-start justify-between">
          <div>
            <div className="flex items-center gap-2">
              <h1 className="text-2xl font-bold">{forum.name}</h1>
              {forum.is_public ? <Globe className="h-5 w-5 text-muted-foreground" /> : <Lock className="h-5 w-5 text-muted-foreground" />}
            </div>
            <p className="mt-1 text-muted-foreground">{forum.description}</p>
            <div className="mt-3 flex gap-4 text-sm text-muted-foreground">
              <span>{formatNumber(forum.members_count)} members</span>
              <span>{formatNumber(forum.topics_count)} topics</span>
            </div>
          </div>
          <ForumActions forum={forum} />
        </div>
      </div>

      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold">Topics</h2>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger asChild>
            <Button size="sm">
              <Plus className="mr-2 h-4 w-4" /> New Topic
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create topic</DialogTitle>
              <DialogDescription>Start a discussion in {forum.name}.</DialogDescription>
            </DialogHeader>
            <div className="space-y-4 py-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Title</label>
                <Input value={form.title} onChange={(e) => setForm((f) => ({ ...f, title: e.target.value }))} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Content</label>
                <textarea
                  className="w-full rounded-md border px-3 py-2 text-sm"
                  rows={4}
                  value={form.content}
                  onChange={(e) => setForm((f) => ({ ...f, content: e.target.value }))}
                />
              </div>
            </div>
            <DialogFooter>
              <Button onClick={handleCreate} disabled={!form.title || !form.content}>Create</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <div className="space-y-3">
        {topicsLoading ? (
          <>
            <Skeleton className="h-24 w-full" />
            <Skeleton className="h-24 w-full" />
          </>
        ) : topics?.length ? (
          topics.map((topic) => (
            <Card key={topic.id}>
              <CardHeader className="pb-2">
                <Link to={`/forums/${forum.id}/topics/${topic.id}`} className="text-lg font-semibold hover:underline">
                  {topic.is_pinned && <span className="mr-2 text-primary">Pinned:</span>}
                  {topic.is_locked && <span className="mr-2 text-muted-foreground">[Locked]</span>}
                  {topic.title}
                </Link>
                <CardDescription>
                  by {topic.user?.display_name || "Unknown"} · {formatDate(topic.created_at)}
                </CardDescription>
              </CardHeader>
              <CardContent className="flex items-center gap-4 text-sm text-muted-foreground">
                <span className="flex items-center gap-1">
                  <MessageSquare className="h-4 w-4" /> {topic.reply_count} replies
                </span>
              </CardContent>
            </Card>
          ))
        ) : (
          <p className="text-center text-muted-foreground">No topics yet.</p>
        )}
      </div>
    </div>
  );
}

function ForumActions({ forum }: { forum: { id: string; is_member?: boolean } }) {
  const join = useJoinForum();
  const leave = useLeaveForum();

  const handleJoin = async () => {
    try {
      await join.mutateAsync(forum.id);
      toast.success("Joined forum");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  const handleLeave = async () => {
    try {
      await leave.mutateAsync(forum.id);
      toast.success("Left forum");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return forum.is_member ? (
    <Button variant="outline" onClick={handleLeave}>Leave</Button>
  ) : (
    <Button onClick={handleJoin}>Join</Button>
  );
}

function Input(props: React.InputHTMLAttributes<HTMLInputElement>) {
  return (
    <input
      {...props}
      className={`flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 ${props.className || ""}`}
    />
  );
}
