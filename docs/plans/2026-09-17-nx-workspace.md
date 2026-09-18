# FORGE Nx Workspace Implementation Plan

> **REQUIRED SUB-SKILL:** Use the executing-plans skill to implement this plan task-by-task.

**Goal:** Add the requested Bun/Nx multi-language workspace to the existing FORGE repository without replacing its Rust program or deploying anything.

**Architecture:** Better-T-Stack generates Next.js and Expo/Uniwind in an ignored staging directory; only new scaffold files are copied into the approved existing root. Nx orchestrates package scripts and native Go/Rust commands without additional framework plugins. The TypeScript SDK packages the existing generated program IDL; backend and banking services start as local health-only shells, not financial implementations.

**Tech Stack:** Bun, Nx, Next.js, Expo/Uniwind, TypeScript, Go standard library, existing Anchor/Rust/Nix tools, Docker configuration for the web app, Oxlint/Ultracite, project-local MCP and skills.

---

## Scope and safety

- User selected the existing `forge/` root; preserve Git history, Cargo files, program source, tests, brand documents and existing shell scripts.
- Keep database, ORM, authentication, payments, API framework, examples and server deployment absent, as requested.
- No chain deployment, Docker deployment, wallet access, signing, financial endpoints, Nx Cloud, global agent configuration or commits.
- Use the Better-T-Stack Nx addon as the single scaffold; do not run a second workspace creator over existing files.

## Task 1: Generate and inspect the scaffold

- Inspect Better-T-Stack's current CLI schema and pin the resolved generator version for replay.
- Generate into `.cache/better-t-stack/forge/` with the requested stack and explicit project-local addon options.
- Delay dependency installation until files are integrated at the root, avoiding two full installs.
- Inspect generated manifests, `nx.json`, Docker files, lint configuration and agent configuration; summarize generated content before adding services.
- Copy only absent root paths. Merge `.gitignore` and README deliberately rather than overwriting either.

## Task 2: Add requested service and SDK boundaries

- Create `apps/backend/` with Bun/TypeScript build, typecheck, local health route and a small request-level regression test. Write and run the test before the implementation.
- Create `services/banking/` with a Go module, standard-library HTTP health handler, timeouts and a small `httptest` regression test. Write and run the test before the implementation.
- Create `packages/sdk/` with IDL/type exports, build/typecheck targets and a smoke test against the generated program artifacts. Do not claim transaction builders exist.
- Add `programs/forge/project.json` wrapping the existing Nix build/test commands; do not change on-chain behavior. Cache only public build outputs, never generated keypairs.
- Configure truthful Nx dependency edges and build ordering. Keep Go independent until actual cross-service integration exists.

## Task 3: Integrate tooling

- Update root `package.json`, `nx.json`, `.gitignore`, generated application manifests and Docker context as needed for the multi-language root.
- Preserve generated-file conventions and add explicit outputs and cache inputs for native builds.
- Add Bun, Node.js and Go to the existing declarative Nix development shell if available from its pinned package set.
- Install from the root with Bun, retaining `bun.lock`.

## Task 4: Validate and document

- Run focused tests first: backend Bun tests, SDK smoke test and `go test ./...` in `services/banking`.
- Check touched source with LSP diagnostics before full builds.
- Run `npx nx run-many -t build`, then relevant typecheck/test/lint targets; retain exact results and any limitations.
- Export `npx nx graph --file=.cache/nx/project-graph.html` and JSON for graph inspection; do not start a public server.
- Check `git diff --check`, final Git status and edited-file diagnostics.
- Update `README.md` with workspace paths, commands, local ports, Docker build command, scaffold provenance and explicit boundaries. Keep existing deployment restrictions intact.

## Completion evidence

- Better-T-Stack 3.44.0 generated the requested frontend/addon combination; installation completed at the root. `bun install --frozen-lockfile` also passed inside the pinned Nix shell (Bun 1.4.2).
- `npx nx run-many -t build` passed for all six buildable projects; the repeat run used six local cache hits. Graph contains eight projects including the shared UI/config packages, without extra language plugins.
- `npx nx run-many -t build check-types test --parallel=1 --output-style=static` passed. Tests cover 10 existing Rust vault cases, Go health routing, TypeScript backend health routing, SDK runtime/declaration artifact packaging, and the Expo build lifecycle guard.
- SDK import passed in Node ESM, and a TypeScript NodeNext consumer compiled with `tsc --ignoreConfig --noEmit --module nodenext --moduleResolution nodenext --target esnext --resolveJsonModule --types node .cache/sdk-consumer.mts`.
- `bun run check`, `expo install --check` from `apps/native`, `docker compose config --quiet`, and `git diff --check` passed. Edited-file diagnostics reported no blocking errors.
- Loopback HTTP smoke checks returned 200 for the built Next.js app and both built service executables. All smoke-test processes were stopped.
- Graph exports: `.cache/nx/project-graph.html` and `.cache/nx/project-graph.json`. Detailed build/check/test output: `.cache/nx-validation-final.log` (local, ignored).

### Corrections discovered during validation

- The generator's old Expo skill name no longer exists. Installed the current official `expo-overview` and `expo-native-ui` skills alongside React guidance; updated metadata and skill lock.
- Replaced the SDK's split bundle/declaration build with TypeScript emission so declarations resolve their packaged JSON and NodeNext consumers resolve type imports.
- Bun ran the generator's `prebuild` script before bundle builds, creating/regenerating ignored iOS/Android folders. Renamed it to explicit `native:generate`; a Node standard-library regression test prevents accidental lifecycle regeneration. Final build logs contain no Expo prebuild invocation. Generated platform folders remain ignored; no native binary was signed or installed.
- Scoped lint compatibility to generator function styles and Expo CommonJS/render-prop conventions. Removed unused imports and an inaccessible pointer-only input-group shortcut.

### Remaining boundaries

Docker image build/container execution, device/emulator execution and financial integrations are not validated or implemented. No program was deployed, no wallet was accessed, and no commit or push was performed. Existing Rust source, Cargo/Anchor configuration, tests and build scripts are unchanged.
