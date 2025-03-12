# dual cross compile toolchain
# https://github.com/noteed/riscv-hello-asm/blob/main/shell.nix
{
  description = "binary-room";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };


  outputs = { self, nixpkgs, fenix, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ fenix.overlays.default ];
          };
          rustToolchain = fenix.packages."${system}".stable;
        in
        {
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              (rustToolchain.withComponents
              [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ])
            ];
            packages = with pkgs; [
              rust-analyzer-nightly
              clang-tools
              hyperfine # benchmarking tool
            ];
          };
        }
      );
}
