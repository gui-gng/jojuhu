import { useState } from "react";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { useForums, useCreateForum, useJoinForum, useLeaveForum } from "@/hooks/use-api";
import { Skeleton } from "@/components/ui/skeleton";
import { Users, Lock, Globe, Plus } from "lucide-react";
import { toast } from "sonner";
import { formatNumber } from "@/lib/utils";

export function ForumsPage() {
  const { data, isLoading } = useForums();
  const createForum = useCreateForum();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState({ name: "", description: "", is_public: true });

  const handleCreate = async () => {
    try {
      await createForum.mutateAsync(form);
      setOpen(false);
      setForm({ name: "", description: "", is_public: true });
      toast.success("Forum created");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Forums</h1>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger asChild>
            <Button>
              <Plus className="mr-2 h-4 w-4" /> Create Forum
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create a forum</DialogTitle>
              <DialogDescription>Start a new community.</DialogDescription>
            </DialogHeader>
            <div className="space-y-4 py-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Name</label>
                <Input value={form.name} onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))} />
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Description</label>
                <Input value={form.description} onChange={(e) => setForm((f) => ({ ...f, description: e.target.value }))} />
              </div>
            </div>
            <DialogFooter>
              <Button onClick={handleCreate} disabled={!form.name || !form.description}>Create</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <div className="grid gap-4 sm:grid-cols-2">
        {isLoading ? (
          <>
            <Skeleton className="h-32 w-full" />
            <Skeleton className="h-32 w-full" />
            <Skeleton className="h-32 w-full" />
            <Skeleton className="h-32 w-full" />
          </>
        ) : data?.length ? (
          data.map((forum) => <ForumCard key={forum.id} forum={forum} />)
        ) : (
          <p className="text-muted-foreground">No forums yet.</p>
        )}
      </div>
    </div>
  );
}

function ForumCard({ forum }: { forum: { id: string; name: string; description: string; is_public: boolean; members_count: number; topics_count: number; is_member?: boolean } }) {
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

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between">
          <CardTitle className="text-lg">{forum.name}</CardTitle>
          {forum.is_public ? <Globe className="h-4 w-4 text-muted-foreground" /> : <Lock className="h-4 w-4 text-muted-foreground" />}
        </div>
        <CardDescription className="line-clamp-2">{forum.description}</CardDescription>
      </CardHeader>
      <CardContent className="flex items-center justify-between">
        <div className="flex items-center gap-3 text-sm text-muted-foreground">
          <span className="flex items-center gap-1">
            <Users className="h-4 w-4" /> {formatNumber(forum.members_count)}
          </span>
          <span>{forum.topics_count} topics</span>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" asChild>
            <Link to={`/forums/${forum.id}`}>View</Link>
          </Button>
          {forum.is_member ? (
            <Button size="sm" variant="secondary" onClick={handleLeave}>Leave</Button>
          ) : (
            <Button size="sm" onClick={handleJoin}>Join</Button>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
