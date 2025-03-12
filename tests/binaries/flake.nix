{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };


  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        # inherit system;
        system = "x86_64-linux";
        crossSystem.config = "aarch64-linux-gnu";
      };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        # Use the same mkShell as documented above
        packages = with pkgs; [
          gcc
          # pkgs.clang-tools
        ];
      };
    };
}
