use std::fs;

use crate::{instruction::RiscVInstruction, translate::translate_instrs};


pub fn translate_to_file(instrs: Vec<RiscVInstruction>, path: String) {
    let arm_instrs = translate_instrs(instrs);
    let mut contents = String::new();
    for instr in arm_instrs {
        let x: String = instr.into();
        contents.push_str(&x);
        contents.push_str("\n");
    }
    fs::write(path, contents).expect("Unable to write file");
}