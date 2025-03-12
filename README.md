# binary-room

Binary translator from RISC-V to ARM written in Rust


## Testing

To get all the cross-compiling dependencies, enter the nix shell using `nix develop`.

`cargo test -- --nocapture`

Then run `./run.sh filename` to assemble and run the file.
