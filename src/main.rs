use std::fs;

fn binary_translate(riscv_asm: &str) -> &str {
    "noop"
}

fn main() {
    // Hard code the arguments for now.
    let path = "./test/hello_world.s";
    let output_path = "./test/hello_world_translated.s";
    let riscv_asm = fs::read_to_string(path).expect("Unable to read file");

    let translated_asm = binary_translate(&riscv_asm);
    fs::write(output_path, translated_asm).expect("Unable to write file");
}
