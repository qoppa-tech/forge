# Also supports an unstaged workspace without copying ignored build artifacts.
let
  lock = builtins.fromJSON (builtins.readFile ./flake.lock);
  nixpkgs = builtins.fetchTree lock.nodes.nixpkgs.locked;
  flake = (import ./flake.nix).outputs { inherit nixpkgs; };
in flake.devShells.x86_64-linux.default
