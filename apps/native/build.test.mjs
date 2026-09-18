import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

test("bundle builds never trigger Expo native-folder generation", () => {
  const { scripts } = JSON.parse(
    readFileSync(new URL("package.json", import.meta.url), "utf-8")
  );
  assert.equal(scripts.prebuild, undefined);
  assert.match(scripts.build, /^expo export /u);
  assert.equal(scripts["native:generate"], "expo prebuild");
});
