let
  pkgs = import <nixpkgs>
    {
      crossSystem = (import <nixpkgs/lib>).systems.examples.riscv64;
    };
in
pkgs.mkShell {
  name = "kernel-qemu";
  depsBuildBuild = with pkgs; [
    # Kernel
    gcc
    # gnumake
    # flex
    # bison
    # bc
    # ncurses
    # pkg-config
    # perl
    # # Modules
    # kmod
    # # ramfs
    # cpio
    # #justqemuthings
    # qemu
  ];
}

