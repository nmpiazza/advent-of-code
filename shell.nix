{ pkgs ? import <nixpkgs> { } }:
let
  rust-overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust-overlay ]; };
in
with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    rust-bin.stable."1.82.0".default # Includes cargo and rustc
    bacon
    clippy
    rustfmt
    rust-analyzer
    gcc
  ];
  buildInputs = [
    vimPlugins.coc-json
    vimPlugins.coc-rust-analyzer
  ];
  # https://nixos.wiki/wiki/Rust
  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
