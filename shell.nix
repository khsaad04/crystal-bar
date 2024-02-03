{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gtk4
    gtk4-layer-shell
    glib
    gcc
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
}
