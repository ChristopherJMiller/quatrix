{ pkgs ? import <nixpkgs> {} }:

let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = overrides.toolchain.channel;
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
  };
in

pkgs.mkShell {
  buildInputs = (import ./nix/inputs.nix pkgs).buildInputs ++ [ rust ];  
  RUST_BACKTRACE = 1;
}