{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      packages = { };
      devShells.default = with pkgs; mkShell {
        packages = [
          rust-analyzer
          rustc
          cargo
          clippy
          rustfmt
        ];
      };
    });
}
