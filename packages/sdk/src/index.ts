import forgeIdl from "./generated/forge.json" with { type: "json" };

export { default as forgeIdl } from "./generated/forge.json" with { type: "json" };
export type { Forge } from "./generated/forge.js";
export const FORGE_PROGRAM_ID = forgeIdl.address;
