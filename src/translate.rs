use crate::instruction::{ArmInstruction, RiscVInstruction};

/// Run the core logic to match from RISC-V to ARM Instructions.

/// Translate one instruction at a time.
pub fn translate(riscv_instr: RiscVInstruction) -> ArmInstruction {
    match riscv_instr {
        RiscVInstruction::Addi => add,
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
