{
  description = "Run a process with a decorated stdout/stderr";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
      in
      rec {
        packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "stdecor";
            version = "0.1.6";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );
}
