import { useState } from "react";
import { Link } from "react-router-dom";
import { Input } from "@/components/ui/input";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSearch, useTrendingHashtags } from "@/hooks/use-api";
import { Skeleton } from "@/components/ui/skeleton";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Search as SearchIcon, TrendingUp } from "lucide-react";
import { PostCard } from "@/components/post-card";

export function SearchPage() {
  const [q, setQ] = useState("");
  const [type, setType] = useState("all");
  const { data, isLoading } = useSearch(q, type);
  const { data: trending } = useTrendingHashtags();

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Search</h1>

      <div className="relative">
        <SearchIcon className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder="Search posts, users, forums, topics..."
          className="pl-10"
          value={q}
          onChange={(e) => setQ(e.target.value)}
        />
      </div>

      {!q && (
        <div className="rounded-xl border bg-card p-4">
          <h2 className="mb-3 flex items-center gap-2 font-semibold">
            <TrendingUp className="h-4 w-4" /> Trending Hashtags
          </h2>
          <div className="flex flex-wrap gap-2">
            {trending?.hashtags?.map((h) => (
              <button
                key={h.name}
                onClick={() => setQ(`#${h.name}`)}
                className="rounded-full bg-muted px-3 py-1 text-sm hover:bg-accent"
              >
                #{h.name}
              </button>
            )) || <p className="text-sm text-muted-foreground">No trending hashtags.</p>}
          </div>
        </div>
      )}

      {q && (
        <Tabs value={type} onValueChange={setType}>
          <TabsList className="w-full">
            <TabsTrigger value="all">All</TabsTrigger>
            <TabsTrigger value="posts">Posts</TabsTrigger>
            <TabsTrigger value="users">Users</TabsTrigger>
            <TabsTrigger value="forums">Forums</TabsTrigger>
          </TabsList>

          <div className="mt-4">
            {isLoading ? (
              <Skeleton className="h-40 w-full" />
            ) : (
              <SearchResults data={data} type={type} />
            )}
          </div>
        </Tabs>
      )}
    </div>
  );
}

function SearchResults({ data, type }: { data?: { posts?: any[]; users?: any[]; forums?: any[]; topics?: any[] }; type: string }) {
  if (!data) return <p className="text-muted-foreground">Start typing to search.</p>;

  const posts = type === "all" || type === "posts" ? data.posts : [];
  const users = type === "all" || type === "users" ? data.users : [];
  const forums = type === "all" || type === "forums" ? data.forums : [];

  const hasResults = (posts?.length || 0) + (users?.length || 0) + (forums?.length || 0) > 0;
  if (!hasResults) return <p className="text-muted-foreground">No results found.</p>;

  return (
    <div className="space-y-4">
      {posts?.map((post) => <PostCard key={post.id} post={post} />)}
      {users?.map((user) => (
        <Link
          key={user.id}
          to={`/u/${user.username}`}
          className="flex items-center gap-3 rounded-xl border bg-card p-4 hover:bg-accent"
        >
          <Avatar className="h-10 w-10">
            <AvatarImage src={user.avatar_url} />
            <AvatarFallback>{user.display_name?.[0] || "?"}</AvatarFallback>
          </Avatar>
          <div>
            <p className="font-medium">{user.display_name}</p>
            <p className="text-sm text-muted-foreground">@{user.username}</p>
          </div>
        </Link>
      ))}
      {forums?.map((forum) => (
        <Link
          key={forum.id}
          to={`/forums/${forum.id}`}
          className="block rounded-xl border bg-card p-4 hover:bg-accent"
        >
          <p className="font-medium">{forum.name}</p>
          <p className="text-sm text-muted-foreground">{forum.description}</p>
        </Link>
      ))}
    </div>
  );
}
