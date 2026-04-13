import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { usePrivacySettings, useUpdatePrivacy } from "@/hooks/use-api";
import { Skeleton } from "@/components/ui/skeleton";
import { toast } from "sonner";
import { Moon, Sun, Laptop } from "lucide-react";
import { useThemeStore } from "@/store/theme";

export function SettingsPage() {
  const { data: privacy, isLoading } = usePrivacySettings();
  const updatePrivacy = useUpdatePrivacy();
  const theme = useThemeStore((s) => s.theme);
  const setTheme = useThemeStore((s) => s.setTheme);

  const toggle = async (key: keyof import("@/lib/types").PrivacySettings, value: boolean) => {
    try {
      await updatePrivacy.mutateAsync({ [key]: value });
      toast.success("Setting updated");
    } catch (err: any) {
      toast.error(err.message);
    }
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Settings</h1>

      <Card>
        <CardHeader>
          <CardTitle>Appearance</CardTitle>
          <CardDescription>Choose your preferred theme.</CardDescription>
        </CardHeader>
        <CardContent className="flex gap-2">
          <Button variant={theme === "light" ? "default" : "outline"} onClick={() => setTheme("light")}>
            <Sun className="mr-2 h-4 w-4" /> Light
          </Button>
          <Button variant={theme === "dark" ? "default" : "outline"} onClick={() => setTheme("dark")}>
            <Moon className="mr-2 h-4 w-4" /> Dark
          </Button>
          <Button variant={theme === "system" ? "default" : "outline"} onClick={() => setTheme("system")}>
            <Laptop className="mr-2 h-4 w-4" /> System
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Privacy</CardTitle>
          <CardDescription>Control your privacy preferences.</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {isLoading || !privacy ? (
            <>
              <Skeleton className="h-8 w-full" />
              <Skeleton className="h-8 w-full" />
            </>
          ) : (
            <>
              <div className="flex items-center justify-between">
                <div>
                  <p className="font-medium">Private Profile</p>
                  <p className="text-sm text-muted-foreground">Only approved followers can see your posts.</p>
                </div>
                <Switch
                  checked={privacy.is_profile_private}
                  onCheckedChange={(v) => toggle("is_profile_private", v)}
                />
              </div>
              <div className="flex items-center justify-between">
                <div>
                  <p className="font-medium">Show Activity Status</p>
                  <p className="text-sm text-muted-foreground">Let others know when you're online.</p>
                </div>
                <Switch
                  checked={privacy.show_activity_status}
                  onCheckedChange={(v) => toggle("show_activity_status", v)}
                />
              </div>
              <div className="flex items-center justify-between">
                <div>
                  <p className="font-medium">DMs from Anyone</p>
                  <p className="text-sm text-muted-foreground">Allow direct messages from anyone.</p>
                </div>
                <Switch
                  checked={privacy.allow_dm_from_anyone}
                  onCheckedChange={(v) => toggle("allow_dm_from_anyone", v)}
                />
              </div>
            </>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

function Switch({ checked, onCheckedChange }: { checked: boolean; onCheckedChange: (v: boolean) => void }) {
  return (
    <button
      onClick={() => onCheckedChange(!checked)}
      className={`relative h-6 w-11 rounded-full transition-colors ${checked ? "bg-primary" : "bg-muted"}`}
    >
      <span
        className={`absolute left-1 top-1 h-4 w-4 rounded-full bg-white transition-transform ${checked ? "translate-x-5" : ""}`}
      />
    </button>
  );
}
