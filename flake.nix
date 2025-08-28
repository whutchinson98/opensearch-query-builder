{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          system = system;
        };
        packages = with pkgs; [
          cargo-info
          cargo-udeps
          just
          rustc
          cargo
          clippy
          rustfmt
          rust-analyzer
        ];

        libraries = [
        ];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages ++ libraries;
        };
      }
    );
}
