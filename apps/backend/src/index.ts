import { app } from "./app";

Bun.serve({
  ...app,
  hostname: Bun.env.HOST ?? "127.0.0.1",
  port: Bun.env.PORT ?? 3000,
});
