{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
        libraries = with pkgs;[
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl_3
          librsvg
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            # Rust dev
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })

            # Generate rust plaid client
            openapi-generator-cli

            # JS dev
            nodejs_20

            # Tauri deps
            curl
            wget
            pkg-config
            dbus
            openssl_3
            glib
            gtk3
            libsoup
            webkitgtk
            librsvg
            glib-networking

            # Python dev
            (python3.withPackages (ps: with ps; [ pandas pytorch ]))

            # just for debugging
            sqlite
            rlwrap
          ];

          shellHook =
            ''
              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
              export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
            '';
        };

        packages.default = rustPlatform.buildRustPackage rec {
          pname = "finance-app";
          version = "0.1.0";

          src = builtins.path {
            path = ./.;
            name = "finance-app";
          };

          sourceRoot = "finance-app/src-tauri";

          cargoLock = {
            lockFile = ./src-tauri/Cargo.lock;
          };

          nativeBuildInputs = [
            copyDesktopItems
            wrapGAppsHook
            pkg-config
          ];

          buildInputs = [
            openssl
            dbus
            glib
            glib-networking
            libayatana-appindicator
            webkitgtk
          ];
        };
      }
    );
}
