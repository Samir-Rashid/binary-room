use std::fs;
use std::str::FromStr;
mod instruction;
pub mod translate;
use instruction::RiscVInstruction;

/// Parse a text file into our enum.
fn parse_asm(asm: &str) -> Vec<RiscVInstruction> {
    asm.lines()
        .filter_map(|line| {
            // TODO (Samir): Not sure that this will handle assembly labels
            // We probably need to construct a map for those to find the
            // original instruction they map to.
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                None
            } else {
                RiscVInstruction::from_str(parts[0]).ok()
            }
        })
        .collect()
}

/// Runs binary translation
///   text file -> [`Instruction`] enum array -> text file
fn binary_translate(riscv_asm: &str) -> String {
    let instructions = parse_asm(riscv_asm);
    instructions
        .into_iter()
        .map(|instr| format!("{:?}", instr))
        .collect::<Vec<String>>()
        .join("\n")
}

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
