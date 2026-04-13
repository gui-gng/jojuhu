import { useState } from "react";
import { useFollowingStories, useMyStories, useCreateStory, useViewStory } from "@/hooks/use-api";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Skeleton } from "@/components/ui/skeleton";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog";
import { toast } from "sonner";
import { formatDate } from "@/lib/utils";
import { Plus, X } from "lucide-react";

export function StoriesPage() {
  const { data: following, isLoading: fLoading } = useFollowingStories();
  const { data: mine, isLoading: mLoading } = useMyStories();
  const [viewing, setViewing] = useState<{ story: any; user: any } | null>(null);

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Stories</h1>

      <div className="rounded-xl border bg-card p-4">
        <h2 className="mb-3 font-semibold">Following</h2>
        {fLoading ? (
          <div className="flex gap-3">
            <Skeleton className="h-16 w-16 rounded-full" />
            <Skeleton className="h-16 w-16 rounded-full" />
          </div>
        ) : following?.stories?.length ? (
          <div className="flex gap-4 overflow-x-auto pb-2">
            {groupByUser(following.stories).map(({ user, stories }) => (
              <button
                key={user.id}
                onClick={() => setViewing({ story: stories[0], user })}
                className="flex flex-col items-center gap-1"
              >
                <Avatar className="h-16 w-16 ring-2 ring-primary ring-offset-2">
                  <AvatarImage src={user.avatar_url} />
                  <AvatarFallback>{user.display_name?.[0] || "?"}</AvatarFallback>
                </Avatar>
                <span className="max-w-[4rem] truncate text-xs">{user.display_name}</span>
              </button>
            ))}
          </div>
        ) : (
          <p className="text-sm text-muted-foreground">No stories from people you follow.</p>
        )}
      </div>

      <div className="rounded-xl border bg-card p-4">
        <div className="mb-3 flex items-center justify-between">
          <h2 className="font-semibold">My Stories</h2>
          <CreateStoryButton />
        </div>
        {mLoading ? (
          <div className="flex gap-3">
            <Skeleton className="h-16 w-16 rounded-full" />
          </div>
        ) : mine?.stories?.length ? (
          <div className="flex gap-4 overflow-x-auto pb-2">
            {mine.stories.map((story) => (
              <div key={story.id} className="flex flex-col items-center gap-1">
                <Avatar className="h-16 w-16">
                  <AvatarImage src={story.media_url} />
                  <AvatarFallback>Me</AvatarFallback>
                </Avatar>
                <span className="text-xs text-muted-foreground">{formatDate(story.created_at)}</span>
              </div>
            ))}
          </div>
        ) : (
          <p className="text-sm text-muted-foreground">You haven't posted any stories yet.</p>
        )}
      </div>

      {viewing && (
        <StoryViewer
          story={viewing.story}
          user={viewing.user}
          onClose={() => setViewing(null)}
        />
      )}
    </div>
  );
}

function groupByUser(stories: any[]) {
  const map = new Map<string, { user: any; stories: any[] }>();
  for (const s of stories) {
    const uid = s.user_id;
    if (!map.has(uid)) map.set(uid, { user: s.user, stories: [] });
    map.get(uid)!.stories.push(s);
  }
  return Array.from(map.values());
}

function CreateStoryButton() {
  const create = useCreateStory();
  const [url, setUrl] = useState("");
  const [open, setOpen] = useState(false);

  const handleCreate = async () => {
    if (!url.trim()) return;
    try {
      await create.mutateAsync({ media_url: url.trim(), media_type: "image" });
      setOpen(false);
      setUrl("");
      toast.success("Story created");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <>
      <Button size="sm" variant="outline" onClick={() => setOpen(true)}>
        <Plus className="mr-1 h-4 w-4" /> Add
      </Button>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogTitle>Create Story</DialogTitle>
          <div className="py-4">
            <input
              className="w-full rounded-md border px-3 py-2 text-sm"
              placeholder="Image URL"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
            />
          </div>
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setOpen(false)}>Cancel</Button>
            <Button onClick={handleCreate} disabled={!url.trim()}>Post</Button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}

function StoryViewer({ story, user, onClose }: { story: any; user: any; onClose: () => void }) {
  const view = useViewStory();

  useState(() => {
    view.mutate(story.id);
  });

  return (
    <Dialog open onOpenChange={onClose}>
      <DialogContent className="max-w-lg border-none bg-black p-0 text-white">
        <div className="relative flex h-[600px] flex-col">
          <button onClick={onClose} className="absolute right-2 top-2 z-10 rounded-full bg-black/50 p-1">
            <X className="h-5 w-5" />
          </button>
          <img src={story.media_url} alt="" className="h-full w-full object-contain" />
          <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-4">
            <div className="flex items-center gap-2">
              <Avatar className="h-8 w-8">
                <AvatarImage src={user?.avatar_url} />
                <AvatarFallback>{user?.display_name?.[0] || "?"}</AvatarFallback>
              </Avatar>
              <span className="font-medium">{user?.display_name}</span>
            </div>
            {story.caption && <p className="mt-1 text-sm">{story.caption}</p>}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
