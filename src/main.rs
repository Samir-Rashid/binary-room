use std::fs;
use std::str::FromStr;
mod instruction;
use instruction::Instruction;

fn parse_asm(asm: &str) -> Vec<Instruction> {
    asm.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                None
            } else {
                Instruction::from_str(parts[0]).ok()
            }
        })
        .collect()
}

fn binary_translate(riscv_asm: &str) -> String {
    let instructions = parse_asm(riscv_asm);
    instructions
        .into_iter()
        .map(|instr| format!("{:?}", instr))
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    // Hard code the arguments for now.
    let path = "./test/hello_world.s";
    let output_path = "./test/hello_world_translated.s";
    let riscv_asm = fs::read_to_string(path).expect("Unable to read file");

    let translated_asm = binary_translate(&riscv_asm);
    fs::write(output_path, translated_asm).expect("Unable to write file");
}
