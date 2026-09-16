#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Keep downloaded Solana build tools outside host-managed toolchains.
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export HOME="$PWD/.tools/home"
export PATH="$PWD/.tools/bin:$PATH"
mkdir -p "$HOME"
cargo-build-sbf --install-only --tools-version v1.57
export PATH="$HOME/.cache/solana/v1.57/platform-tools/rust/bin:$PATH"
unset RUSTC
# SBPF v0 remains compatible with the Anchor 1.2 LiteSVM template runtime.
anchor build --no-idl --ignore-keys --tools-version v1.57 --arch v0 -- --no-rustup-override -- --locked
mkdir -p target/idl target/types
anchor idl build --out target/idl/forge.json --out-ts target/types/forge.ts -- --locked
