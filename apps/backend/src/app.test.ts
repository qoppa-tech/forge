import { afterAll, expect, test } from "bun:test";

const entry = new URL("app.ts", import.meta.url);
let server: ReturnType<typeof Bun.serve> | undefined;
afterAll(() => server?.stop(true));

test("backend exposes only liveness and rejects unknown or write routes", async () => {
  expect(await Bun.file(entry).exists()).toBe(true);
  const { app } = await import(entry.href);
  server = Bun.serve({ ...app, hostname: "127.0.0.1", port: 0 });
  const response = await fetch(new URL("/health", server.url));
  expect(response.status).toBe(200);
  const health = await response.json();
  const { FORGE_PROGRAM_ID } = await import("@forge/sdk");
  expect(health).toEqual({
    programId: FORGE_PROGRAM_ID,
    service: "backend",
    status: "ok",
  });
  const missing = await fetch(new URL("/missing", server.url));
  const write = await fetch(new URL("/health", server.url), { method: "POST" });
  expect(missing.status).toBe(404);
  expect(write.status).toBe(404);
});
