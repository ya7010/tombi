{
  description = "Nix-flake development environment for my personal website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    wild = {
      url = "github:davidlattimore/wild";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      wild,
    }:
    let
      system = "x86_64-linux";
      overlays = [
        (import rust-overlay)
        wild.overlays.default
      ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      wildStdenv = pkgs.useWildLinker pkgs.stdenv;
    in
    with pkgs;
    {
      devShells.${system}.default = mkShell.override { stdenv = wildStdenv; } {
        buildInputs = [
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          clang
          openssl
          pkg-config
        ];
      };
    };
}
