{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs = {
      # Randomly chosen commit on 2025-12-11 to have a
      # truely reproducible environment with cargo, rustc, etc. in
      # specific versions. New commit version can be found on
      # https://github.com/NixOS/nixpkgs/tree/nixos-unstable-small
      url = "github:NixOS/nixpkgs/6f313d8e9be4d7db523962ecc3ce97c951bacb1f";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [
            # Rust general
            cargo
            rustc
            rustfmt
            rustPackages.clippy
            # Dev Tools
            just
            prek

            # WASM
            lld
            wasm-pack
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
