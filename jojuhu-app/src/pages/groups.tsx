import { useState } from "react";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { useGroups, useCreateGroup, useJoinGroup, useLeaveGroup } from "@/hooks/use-api";
import { Skeleton } from "@/components/ui/skeleton";
import { Users, Lock, Globe, Plus } from "lucide-react";
import { toast } from "sonner";
import { formatNumber } from "@/lib/utils";

export function GroupsPage() {
  const { data, isLoading } = useGroups();
  const createGroup = useCreateGroup();
  const [open, setOpen] = useState(false);
  const [form, setForm] = useState({ name: "", description: "", is_public: true });

  const handleCreate = async () => {
    try {
      await createGroup.mutateAsync(form);
      setOpen(false);
      setForm({ name: "", description: "", is_public: true });
      toast.success("Group created");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Groups</h1>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger asChild>
            <Button>
              <Plus className="mr-2 h-4 w-4" /> Create Group
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create a group</DialogTitle>
              <DialogDescription>Start a new group.</DialogDescription>
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
          data.map((group) => <GroupCard key={group.id} group={group} />)
        ) : (
          <p className="text-muted-foreground">No groups yet.</p>
        )}
      </div>
    </div>
  );
}

function GroupCard({ group }: { group: { id: string; name: string; description: string; is_public: boolean; member_count: number; is_member?: boolean } }) {
  const join = useJoinGroup();
  const leave = useLeaveGroup();

  const handleJoin = async () => {
    try {
      await join.mutateAsync(group.id);
      toast.success("Joined group");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  const handleLeave = async () => {
    try {
      await leave.mutateAsync(group.id);
      toast.success("Left group");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-start justify-between">
          <CardTitle className="text-lg">{group.name}</CardTitle>
          {group.is_public ? <Globe className="h-4 w-4 text-muted-foreground" /> : <Lock className="h-4 w-4 text-muted-foreground" />}
        </div>
        <CardDescription className="line-clamp-2">{group.description}</CardDescription>
      </CardHeader>
      <CardContent className="flex items-center justify-between">
        <div className="flex items-center gap-3 text-sm text-muted-foreground">
          <span className="flex items-center gap-1">
            <Users className="h-4 w-4" /> {formatNumber(group.member_count)}
          </span>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" asChild>
            <Link to={`/groups/${group.id}`}>View</Link>
          </Button>
          {group.is_member ? (
            <Button size="sm" variant="secondary" onClick={handleLeave}>Leave</Button>
          ) : (
            <Button size="sm" onClick={handleJoin}>Join</Button>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
