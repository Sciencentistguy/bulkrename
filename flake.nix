{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
      rustPlatform = pkgs.makeRustPlatform {
        cargo = pkgs.cargo;
        rustc = pkgs.rustc;
      };
    in {
      packages.default = rustPlatform.buildRustPackage {
        pname = "bulkrename";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        meta = with pkgs.lib; {
          description = "A CLI tool for bulk renaming files";
          license = licenses.mit;
        };
      };
    });
}
