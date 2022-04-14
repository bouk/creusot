{
  description = "Creusot is a tool for deductive verification of Rust code";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ rust-overlay.overlay ];
      config.allowUnfree = true;
    };

    rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain);

    in rec {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          alt-ergo
          rust
          why3
          z3
          zlib
        ];
      };
    }
  );
}
