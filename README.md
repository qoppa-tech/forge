# FORGE

**Banking infrastructure. Onchain.**

Proprietary developer sandbox for traditional-bank workflows on Solana. This checkout implements **milestone 1 only**: creation and treasury funding of a PDA-controlled vault. It is not a banking service, audited protocol, or production-ready custody system.

## Implemented boundary

- One Anchor program; `create_vault` and `fund_vault` only.
- Treasury signs creation and funding. Two distinct approver public keys, limits, mint and treasury destination are recorded immutably.
- Vault PDA: `["vault", treasury, 32-byte vault ID]`. Its legacy SPL token account uses `["tokens", vault]`; SPL Token owns that account, while the vault PDA is its transfer authority.
- Treasury explicitly selects a six-decimal legacy mint at creation. That address is fixed for the vault, not a global issuer allowlist or proof of bank identity. Wrong mints, owners, token programs and cross-vault account substitutions fail.
- Funding uses checked integer token transfers. The fixture funds **10,000 FORGE_TEST_USD = 10,000,000,000 base units**.

**Funds cannot leave the vault at this milestone.** Withdrawal, loan approval, draw, repayment and pause instructions do not exist yet. Approver keys and lending limits are stored for the next milestone; no lending policy is claimed operational. Never fund with assets of real value.

## Build and test without deployment

Tested platform: Nix on x86_64 Linux. `flake.lock` pins Anchor 1.2.0, Solana CLI 4.0.3 and host tools. `shell.nix` uses that same lock and also works before the initial Git commit. The development shell pins `NIX_PATH` to this locked source, including the SBF builder's nested NixOS dependency lookup. On NixOS, the downloaded upstream platform-tools executables require `nix-ld` support; the tested host already provides it. This repository does not change system or Home Manager configuration.

```sh
nix-shell
bash scripts/check-nix-path.sh
bash scripts/bootstrap.sh
bash scripts/build.sh
cargo test --locked -p forge
cargo fmt --all -- --check
cargo clippy --locked -p forge --all-targets -- -D warnings
```

Bootstrap installs the separately published `cargo-build-sbf` **4.3.0** under ignored `.tools/`, never over the host-managed Rust installation. The build script downloads platform-tools **v1.57** under `.tools/home/`, uses its Rust compiler, compiles SBPF v0 for the pinned LiteSVM runtime, and generates `target/deploy/forge.so`, `target/idl/forge.json` and `target/types/forge.ts`. First setup requires network access and several GB of disk space. Subsequent commands use the pinned lockfiles and caches.

Always rebuild before running Rust tests after program changes: the tests execute `target/deploy/forge.so`, not a substitute native handler. LiteSVM 0.10.0 verifies signed transactions and executes the actual compiled program plus SPL Token CPIs. The suite checks exact balances and ownership, invalid configuration, absent or forged signatures, cross-vault isolation, Token-2022 rejection, duplicate IDs, zero/insufficient funding and direct SPL theft attempts.

### Reproducible local demonstration

After building, run the in-process fixture:

```sh
cargo test --locked -p forge --test vault create_and_fund_exact_test_token_balance -- --exact --nocapture
```

It creates independent treasury and two approver wallets, issues the disclosed test supply, creates the vault and funds it. Output contains only public role identities, the configured mint, vault/token-account addresses, the funding signature and integer balances. Expected treasury cash: `0`; vault cash: `10000000000`. Wallets are random and held only in memory; addresses and signatures change each run, while the accounting assertions remain reproducible. The signature is an in-process test receipt, not a finalized network transaction or explorer link.

A network seed CLI is deferred until the local-validator deployment step is explicitly approved. No JavaScript application/client dependencies are retained; the generated IDL and TypeScript type are build outputs for milestone 3's SDK.

**LiteSVM is not a local validator.** No deployment or local-validator acceptance is claimed. Do not run `anchor test` as a deployment-free test command: it automatically deploys. Local-validator deployment/test execution requires separate explicit approval; loans remain out of scope until that gate passes.

## Trust and operational limits

- `FORGE_TEST_USD` is a fixture label, not token metadata, redeemable currency, certified security or transferable loan instrument. Fixtures issue test supply explicitly; funding does not mint tokens. The fixture treasury retains mint authority and can issue additional test tokens. The mint has no freeze authority.
- Fixtures use independently generated demonstration treasury and approver wallets held only in memory. No real bank/customer data is required. Generated build keys stay under ignored `target/` and must never be reused outside disposable local testing.
- Creating a vault does not certify a bank, and the rules restrict FORGE instructions, not every transfer of the token. Direct token donations can increase cash; no loan allocation or reconciler is implemented yet.
- There is no deployed program or established upgrade authority in this checkout. If deployed with the upgradeable loader later, the chosen deployment wallet retains upgrade control unless explicitly changed. The prototype must not claim the deployer has no control. `Anchor.toml` names the demonstration treasury wallet by default; deployment authority selection must be recorded at that approved step.
- Builds preserve the declared test program ID using `--ignore-keys`. Generated program keypairs are local artifacts, not portable deployment identities; verify/synchronize the chosen ID and keypair during an approved deployment workflow.
- No API, server, database, SDK package, borrower wallet, dashboard or loan lifecycle is delivered in this milestone.

See [validation evidence and remaining gates](docs/validation.md), [product specification](docs/FORGE-MVP-v0.1.md), and [implementation plan](docs/plans/2026-09-16-vault.md).

## Brand and licensing

[DESIGN.md](DESIGN.md) and [docs/brand/](docs/brand/) preserve the supplied visual identity. Logo concepts and HTML are references, not a working operator interface or finalized vector mark. No screens were built.

Application source is proprietary; no open-source license is granted. The MIT notice in `docs/brand/REFERENCE-LICENSE.txt` applies to the referenced brand collection, not the application. Dependency licenses remain their own.
