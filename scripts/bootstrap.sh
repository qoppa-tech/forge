#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Nixpkgs' solana-cli does not include this separately published build tool.
if [[ ! -x .tools/bin/cargo-build-sbf ]]; then
	cargo install --locked --version 4.3.0 --root "$PWD/.tools" cargo-build-sbf
fi
if [[ "$(.tools/bin/cargo-build-sbf --version)" != 'cargo-build-sbf 4.3.0'* ]]; then
	printf '%s\n' 'Expected project-local cargo-build-sbf 4.3.0; refusing to overwrite another toolchain.' >&2
	exit 1
fi
.tools/bin/cargo-build-sbf --version
