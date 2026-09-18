/// <reference types="bun" />
import { expect, test } from "bun:test";

const root = new URL("../../", import.meta.url);

test("SDK exports the built on-chain IDL without changing its address or instructions", async () => {
  const artifact = await Bun.file(
    new URL("target/idl/forge.json", root)
  ).json();
  const packagedIdl = Bun.file(
    new URL("dist/generated/forge.json", import.meta.url)
  );
  expect(await packagedIdl.exists()).toBe(true);
  expect(await packagedIdl.json()).toEqual(artifact);
  const entry = new URL("dist/index.js", import.meta.url);
  expect(await Bun.file(entry).exists()).toBe(true);
  const { FORGE_PROGRAM_ID, forgeIdl } = await import(entry.href);
  expect(forgeIdl).toEqual(artifact);
  expect(FORGE_PROGRAM_ID).toBe(artifact.address);
  expect(
    forgeIdl.instructions.map(
      (instruction: { name: string }) => instruction.name
    )
  ).toEqual(["create_vault", "fund_vault"]);
});
