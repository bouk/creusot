{
  description = "Creusot is a tool for deductive verification of Rust code";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlay ]; };
    in rec {
      devShell = with pkgs; mkShell {
        buildInputs = [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain)
          zlib
        ];
      };
    }
  );
}
