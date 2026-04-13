import { Client, Stronghold } from "@tauri-apps/plugin-stronghold";
import { appLocalDataDir } from "@tauri-apps/api/path";

let stronghold: Stronghold | null = null;
let client: Client | null = null;

async function getClient(): Promise<Client> {
  if (client) return client;
  const vaultPath = `${await appLocalDataDir()}/stronghold.hold`;
  stronghold = await Stronghold.load(vaultPath, "jojuhu-secure-passphrase");
  client = await stronghold.loadClient("jojuhu_client");
  return client;
}

async function getStore() {
  const c = await getClient();
  return c.getStore();
}

export async function secureSet(key: string, value: string): Promise<void> {
  const store = await getStore();
  const encoder = new TextEncoder();
  await store.insert(key, Array.from(encoder.encode(value)));
  if (stronghold) await stronghold.save();
}

export async function secureGet(key: string): Promise<string | null> {
  try {
    const store = await getStore();
    const data = await store.get(key);
    if (!data) return null;
    const decoder = new TextDecoder();
    return decoder.decode(new Uint8Array(data));
  } catch {
    return null;
  }
}

export async function secureRemove(key: string): Promise<void> {
  try {
    const store = await getStore();
    await store.remove(key);
    if (stronghold) await stronghold.save();
  } catch {
    // ignore
  }
}

export async function setToken(token: string): Promise<void> {
  await secureSet("auth_token", token);
}

export async function getToken(): Promise<string | null> {
  return secureGet("auth_token");
}

export async function removeToken(): Promise<void> {
  await secureRemove("auth_token");
}
