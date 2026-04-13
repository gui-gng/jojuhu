import { useParams, Link } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { useGroup, useGroupMembers, useJoinGroup, useLeaveGroup } from "@/hooks/use-api";
import { Skeleton } from "@/components/ui/skeleton";
import { formatNumber } from "@/lib/utils";
import { ArrowLeft, Lock, Globe } from "lucide-react";
import { toast } from "sonner";

export function GroupDetailPage() {
  const { groupId } = useParams();
  const { data: group, isLoading: groupLoading } = useGroup(groupId || "");
  const { data: members, isLoading: membersLoading } = useGroupMembers(groupId || "");

  if (groupLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-10 w-20" />
        <Skeleton className="h-32 w-full" />
      </div>
    );
  }

  if (!group) return <p className="text-muted-foreground">Group not found.</p>;

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="sm" asChild>
          <Link to="/groups">
            <ArrowLeft className="mr-1 h-4 w-4" /> Back
          </Link>
        </Button>
      </div>

      <div className="rounded-xl border bg-card p-6 shadow-sm">
        <div className="flex items-start justify-between">
          <div>
            <div className="flex items-center gap-2">
              <h1 className="text-2xl font-bold">{group.name}</h1>
              {group.is_public ? <Globe className="h-5 w-5 text-muted-foreground" /> : <Lock className="h-5 w-5 text-muted-foreground" />}
            </div>
            <p className="mt-1 text-muted-foreground">{group.description}</p>
            <div className="mt-3 flex gap-4 text-sm text-muted-foreground">
              <span>{formatNumber(group.member_count)} members</span>
              <span className="capitalize">Your role: {group.my_role || "Guest"}</span>
            </div>
          </div>
          <GroupActions group={group} />
        </div>
      </div>

      <h2 className="text-lg font-semibold">Members</h2>
      <div className="grid gap-3 sm:grid-cols-2">
        {membersLoading ? (
          <>
            <Skeleton className="h-16 w-full" />
            <Skeleton className="h-16 w-full" />
          </>
        ) : members?.members?.length ? (
          members.members.map((m) => (
            <div key={m.user_id} className="flex items-center gap-3 rounded-xl border bg-card p-3">
              <Avatar className="h-10 w-10">
                <AvatarImage src={m.avatar_url} />
                <AvatarFallback>{m.display_name?.[0] || "?"}</AvatarFallback>
              </Avatar>
              <div className="flex-1">
                <p className="font-medium">{m.display_name}</p>
                <p className="text-xs text-muted-foreground">@{m.username}</p>
              </div>
              <span className="rounded-full bg-muted px-2 py-1 text-xs capitalize">{m.role}</span>
            </div>
          ))
        ) : (
          <p className="text-muted-foreground">No members yet.</p>
        )}
      </div>
    </div>
  );
}

function GroupActions({ group }: { group: { id: string; is_member?: boolean } }) {
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

  return group.is_member ? (
    <Button variant="outline" onClick={handleLeave}>Leave</Button>
  ) : (
    <Button onClick={handleJoin}>Join</Button>
  );
}
