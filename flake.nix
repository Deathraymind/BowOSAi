{
  description = "Rust dev shell with latest stable via rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rust-bin.stable.latest.default   # <- rustc/cargo fmt/clippy at latest stable
            pkgs.pkg-config
            pkgs.openssl
            # tauri build depenecies
            pkgs.nodePackages.npm
            pkgs.atk
            pkgs.gdk-pixbuf
            pkgs.cairo
            pkgs.gtk3
            pkgs.webkitgtk_4_1
          ];
        };
      });
}

