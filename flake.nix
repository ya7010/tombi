{
  description = "Nix-flake development environment for my personal website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    crane = {
      url = "github:ipetkov/crane";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      crane,
      flake-utils,
      nixpkgs,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
        );

        tombiPkg = craneLib.buildPackage {
          pname = "tombi";
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          doCheck = true;
          doNotSign = false;
        };
      in
      {
        checks = {
          inherit tombiPkg;
        };

        packages.default = tombiPkg;

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks
          checks = self.checks.${system};

          packages = with pkgs; [
            openssl
            pkg-config
          ];
        };
      }
    );
}
