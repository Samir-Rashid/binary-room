{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };


  outputs = { self, nixpkgs }: let
	  pkgs = import nixpkgs {
            system = "x86_64-linux";
	  };
   in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      # Use the same mkShell as documented above
        packages = with pkgs; [
          # TODO: @Anthony, you can change this to use Fenix if you would like
          rustup
      ];
    };
  };
}
