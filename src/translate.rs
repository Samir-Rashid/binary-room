use crate::instruction::{parse_asm, ArmInstruction, ArmRegister, ArmVal, RiscVInstruction, RiscVRegister};

/// Run the core logic to match from RISC-V to ARM Instructions.

/// Translate one instruction at a time.
pub fn translate(riscv_instr: RiscVInstruction) -> ArmInstruction {
    match riscv_instr {
        RiscVInstruction::Addi {dest, src, imm} => 
            if imm >= 0 {
                ArmInstruction::Add { dest: map_register(dest), arg1: map_register(src), arg2: ArmVal::Imm(imm) }
            } else {
                ArmInstruction::Sub { dest: map_register(dest), arg1: map_register(src), arg2: ArmVal::Imm(imm) }
            },
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

fn map_register(riscv_reg: RiscVRegister) -> ArmRegister {
    todo!()
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
