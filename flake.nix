{
  description = "devShell for crystal-bar";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    fenix,
    flake-utils,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default = let
        overlays = [fenix.overlays.default];
        pkgs = import nixpkgs {
          inherit overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            nativeBuildInputs = [
              pkg-config
              gdb
              gtk4
              gtk4-layer-shell
              (fenix.stable.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
            ];
          };
        };
    });
}
