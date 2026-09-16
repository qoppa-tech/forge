#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Keep nix-shell's own Bash lookup separate from the nested lookup under test.
NIX_BUILD_SHELL=$(command -v bash)
export NIX_BUILD_SHELL
expected=$(nix eval --impure --raw --expr '
  let lock = builtins.fromJSON (builtins.readFile ./flake.lock);
  in (builtins.fetchTree lock.nodes.nixpkgs.locked).outPath
')
for inherited in unset bogus; do
  if [[ "$inherited" == unset ]]; then
    actual=$(env -u NIX_PATH nix-shell --run 'nix-instantiate --find-file nixpkgs')
  else
    actual=$(NIX_PATH=nixpkgs=/forge-nonexistent-nixpkgs nix-shell --run 'nix-instantiate --find-file nixpkgs')
  fi
  if [[ "$actual" != "$expected" ]]; then
    printf 'Nested nixpkgs mismatch (%s): expected %s, got %s\n' "$inherited" "$expected" "$actual" >&2
    exit 1
  fi
  printf 'Nested nixpkgs matches lock (%s inherited NIX_PATH): %s\n' "$inherited" "$actual"
done
