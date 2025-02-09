use std::fs;
use std::str::FromStr;
pub mod instruction;
pub mod translate;
use instruction::RiscVInstruction;



// Samir: I am using main for testing, but it not needed since you can run
// `cargo test` instead.
fn main() {
    // Hard code the arguments for now.
    let path = "../test/binaries/hello_world.s";
    let output_path = "../test/binaries/hello_world_translated.s";
    let riscv_asm = fs::read_to_string(path).expect("Unable to read file");

    let translated_asm = binary_translate(&riscv_asm);
    fs::write(output_path, translated_asm).expect("Unable to write file");
}
