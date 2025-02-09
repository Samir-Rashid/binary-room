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
            if imm >= 0 {
                ArmInstruction::Add {
                    dest: map_register(dest),
                    arg1: map_register(src),
                    arg2: ArmVal::Imm(imm),
                }
            } else {
                ArmInstruction::Sub {
                    dest: map_register(dest),
                    arg1: map_register(src),
                    arg2: ArmVal::Imm(imm),
                }
            }
        }
        RiscVInstruction::S { width, src, dest } => ArmInstruction::Str {
            width: map_width(width),
            src: map_register(src),
            dest: map_val(dest),
        },
        RiscVInstruction::L { width, dest, src } => ArmInstruction::Ldr {
            width: map_width(width),
            dest: map_register(dest),
            src: map_val(src),
        },
        RiscVInstruction::Mv { dest, src } => ArmInstruction::Add {
            dest: map_register(dest),
            arg1: map_register(src),
            arg2: ArmVal::Imm(0),
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
                dest: map_register(dest),
                arg1: ArmRegister {
                    width: ArmWidth::Double,
                    name: ArmRegisterName::Zero,
                },
                arg2: ArmVal::Imm(imm),
            }
        }
    }
}

fn map_register(riscv_reg: RiscVRegister) -> ArmRegister {
    match riscv_reg {
        RiscVRegister::X0 => todo!("Arm doesn't have a zero register"),
        // RiscVRegister::RA => ArmRegister::Lr,
        // RiscVRegister::SP => ArmRegister::Sp,
        // RiscVRegister::GP => ArmRegister::,
        // RiscVRegister::TP => ArmRegister::,
        // RiscVRegister::T0 => ArmRegister::,
        // RiscVRegister::T1 => ArmRegister::,
        // RiscVRegister::T2 => ArmRegister::,
        // RiscVRegister::S0FP => ArmRegister::,
        // RiscVRegister::S1 => ArmRegister::,
        // RiscVRegister::A0 => ArmRegister::,
        // RiscVRegister::A1 => ArmRegister::,
        // RiscVRegister::A2 => ArmRegister::,
        // RiscVRegister::A3 => ArmRegister::,
        // RiscVRegister::A4 => ArmRegister::,
        // RiscVRegister::A5 => ArmRegister::,
        // RiscVRegister::A6 => ArmRegister::,
        // RiscVRegister::A7 => ArmRegister::,
        // RiscVRegister::S2 => ArmRegister::,
        // RiscVRegister::S3 => ArmRegister::,
        // RiscVRegister::S4 => ArmRegister::,
        // RiscVRegister::S5 => ArmRegister::,
        // RiscVRegister::S6 => ArmRegister::,
        // RiscVRegister::S7 => ArmRegister::,
        // RiscVRegister::S8 => ArmRegister::,
        // RiscVRegister::S9 => ArmRegister::,
        // RiscVRegister::S10 => ArmRegister::,
        // RiscVRegister::S11 => ArmRegister::,
        // RiscVRegister::T3 => ArmRegister::,
        // RiscVRegister::T4 => ArmRegister::,
        // RiscVRegister::T5 => ArmRegister::,
        // RiscVRegister::T6 => ArmRegister::,
        // FIXME: do real implementation
        _ => ArmRegister {
            width: ArmWidth::Double,
            name: ArmRegisterName::Sp,
        },
    }
}

fn map_register_name(riscv_reg: RiscVRegister) -> ArmRegisterName {
    // todo!()
        // FIXME: do real implementation
    ArmRegisterName::A1
}

fn map_val(riscv_val: RiscVVal) -> ArmVal {
    match riscv_val {
        RiscVVal::RiscVRegister(riscv_reg) => ArmVal::Reg(map_register(riscv_reg)),
        RiscVVal::Immediate(imm) => ArmVal::Imm(imm),
        RiscVVal::Offset { register, offset } => ArmVal::RegOffset(map_register(register), offset),
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
