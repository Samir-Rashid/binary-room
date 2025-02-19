use core::panic;

use crate::instruction::{
    parse_asm, ArmInstruction, ArmRegister, ArmRegisterName, ArmVal, ArmWidth, RiscVInstruction,
    RiscVRegister, RiscVVal, RiscVWidth,
};

macro_rules! sorry {
    () => {
        todo!()
    };
}

/// Run the core logic to match from RISC-V to ARM Instructions.

/// Translate one instruction at a time.
pub fn translate(riscv_instr: RiscVInstruction) -> ArmInstruction {
    match riscv_instr {
        RiscVInstruction::Addi { dest, src, imm } => {
            let width = RiscVWidth::Word;
            if imm >= 0 {
                ArmInstruction::Add {
                    dest: map_register(dest, width),
                    arg1: map_register(src, width),
                    arg2: ArmVal::Imm(imm),
                }
            } else {
                ArmInstruction::Sub {
                    dest: map_register(dest, width),
                    arg1: map_register(src, width),
                    arg2: ArmVal::Imm(imm),
                }
            }
        }
        RiscVInstruction::S { width, src, dest } => ArmInstruction::Str {
            width: map_width(width),
            src: map_register(src, width),
            dest: map_val(dest, width),
        },
        RiscVInstruction::L { width, dest, src } => ArmInstruction::Ldr {
            width: map_width(width),
            dest: map_register(dest, width),
            src: map_val(src, width),
        },
        RiscVInstruction::Mv { dest, src } =>  {
            let width = RiscVWidth::Double;
            ArmInstruction::Add {
                dest: map_register(dest, width),
                arg1: map_register(src, width),
                arg2: ArmVal::Imm(0),
            }
        },
        RiscVInstruction::Add {
            width,
            dest,
            arg1,
            arg2,
        } => match width {
            RiscVWidth::Word => ArmInstruction::Add {
                dest: ArmRegister {
                    width: ArmWidth::Word,
                    name: map_register_name(dest),
                },
                arg1: ArmRegister {
                    width: ArmWidth::Word,
                    name: map_register_name(arg1),
                },
                arg2: ArmVal::Reg(ArmRegister {
                    width: ArmWidth::Word,
                    name: map_register_name(arg2),
                }),
            },
            RiscVWidth::Double => sorry!(),
        },
        RiscVInstruction::SextW { dest, src } => ArmInstruction::Sxtw {
            dest: ArmRegister {
                width: ArmWidth::Double,
                name: map_register_name(dest),
            },
            src: ArmRegister {
                width: ArmWidth::Word,
                name: map_register_name(src),
            },
        },
        RiscVInstruction::Jr { target } => ArmInstruction::Blr {
            target: map_register_name(target),
        },
        RiscVInstruction::Li { dest, imm } => {
            if imm > 4095 || imm < 0 {
                panic!("Li with imm out of range");
            }

            ArmInstruction::Add {
                dest: map_register(dest, RiscVWidth::Double),
                arg1: ArmRegister {
                    width: ArmWidth::Double,
                    name: ArmRegisterName::Zero,
                },
                arg2: ArmVal::Imm(imm),
            }
        }
    }
}

fn map_register(riscv_reg: RiscVRegister, riscv_width: RiscVWidth) -> ArmRegister {
    ArmRegister {
        width: map_width(riscv_width),
        name: map_register_name(riscv_reg)
    }
}

fn map_register_name(riscv_reg: RiscVRegister) -> ArmRegisterName {
    // todo!()
        // FIXME: do real implementation
    ArmRegisterName::A1
}

fn map_val(riscv_val: RiscVVal, riscv_width: RiscVWidth) -> ArmVal {
    match riscv_val {
        RiscVVal::RiscVRegister(riscv_reg) => ArmVal::Reg(map_register(riscv_reg, riscv_width)),
        RiscVVal::Immediate(imm) => ArmVal::Imm(imm),
        RiscVVal::Offset { register, offset } => ArmVal::RegOffset(map_register(register, riscv_width), offset),
    }
}

fn map_width(riscv_width: RiscVWidth) -> ArmWidth {
    // todo!()
        // FIXME: do real implementation
    ArmWidth::Double
}

// Translate every instruction 1:1
pub fn translate_instrs(riscv_instrs: Vec<RiscVInstruction>) -> Vec<ArmInstruction> {
    riscv_instrs
        .into_iter()
        .map(|instr| translate(instr))
        .collect::<Vec<ArmInstruction>>()
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
