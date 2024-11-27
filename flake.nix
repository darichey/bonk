{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, nixpkgs-unstable, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pkgs-unstable = nixpkgs-unstable.legacyPackages.${system};
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            # Rust dev
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })

            pkg-config
            openssl_3
            tree-sitter

            # Generate rust plaid client
            openapi-generator-cli

            # JS dev
            nodejs_20
            vsce

            # Python dev
            (python3.withPackages (ps: with ps; [ pandas pytorch ]))

            # just for debugging
            sqlite
            rlwrap

            # *AI*
            (pkgs-unstable.ollama.override { # TODO: using unstable until rocm 6 is in stable
              acceleration = "rocm"; # TODO: configurable
            })
          ];
        };
      }
    );
}
