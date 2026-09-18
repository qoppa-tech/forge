import { FORGE_PROGRAM_ID } from "@forge/sdk";

export const app = {
  fetch: () => new Response("Not found", { status: 404 }),
  routes: {
    "/health": {
      GET: () =>
        Response.json({
          programId: FORGE_PROGRAM_ID,
          service: "backend",
          status: "ok",
        }),
    },
  },
};
