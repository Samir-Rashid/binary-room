use crate::instruction::{ArmInstruction, RiscVInstruction, parse_asm};

/// Run the core logic to match from RISC-V to ARM Instructions.

/// Translate one instruction at a time.
pub fn translate(riscv_instr: RiscVInstruction) -> ArmInstruction {
    match riscv_instr {
        RiscVInstruction::Addi => todo!(),
        RiscVInstruction::Sd => todo!(),
        RiscVInstruction::Ld => todo!(),
        RiscVInstruction::Sw => todo!(),
        RiscVInstruction::Lw => todo!(),
        RiscVInstruction::Mv => todo!(),
        RiscVInstruction::Addw => todo!(),
        RiscVInstruction::SextW => todo!(),
        RiscVInstruction::Jr => todo!(),
        RiscVInstruction::Li => todo!(),
    }
}

/// Runs binary translation
///   text file -> [`Instruction`] enum array -> text file
pub fn binary_translate(riscv_asm: &str) -> String {
    let instructions = parse_asm(riscv_asm);
    instructions
        .into_iter()
        .map(|instr| format!("{:?}", instr))
        .collect::<Vec<String>>()
        .join("\n")
}
