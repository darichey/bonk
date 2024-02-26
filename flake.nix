{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
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
          ];
        };
      }
    );
}
