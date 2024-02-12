{
  description = "crystal bar devShell";
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
  };
  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        gtk4
        gtk4-layer-shell
      ];
      nativeBuildInputs = with pkgs; [
        pkg-config
        gdb
      ];
    };
  };
}
