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
pub fn translate(riscv_instr: RiscVInstruction) -> Vec<ArmInstruction> {
    match riscv_instr {
        RiscVInstruction::Addi { dest, src, imm } => {
            if let RiscVRegister::X0 = src {
                return translate(RiscVInstruction::Mvi { dest, imm });
            }

            let width = RiscVWidth::Double;
            if imm >= 0 {
                vec![ArmInstruction::Add {
                    dest: map_register(dest, &width),
                    arg1: map_register(src, &width),
                    arg2: ArmVal::Imm(imm),
                }]
            } else {
                vec![ArmInstruction::Sub {
                    dest: map_register(dest, &width),
                    arg1: map_register(src, &width),
                    arg2: ArmVal::Imm(imm.abs()),
                }]
            }
        }
        RiscVInstruction::Ble { arg1, arg2, target } => vec![{
            let width = RiscVWidth::Double;
            ArmInstruction::Ble {
                arg1: map_register(arg1, &width),
                arg2: map_register(arg2, &width),
                target: map_val(target, &width),
            }
        }],
        RiscVInstruction::J { target } => vec![ArmInstruction::B {
            target: map_val(target, &RiscVWidth::Double),
        }],
        RiscVInstruction::S { width, src, dest } => vec![ArmInstruction::Str {
            width: map_width(&width),
            src: map_register(src, &width),
            dest: map_val(dest, &width),
        }],
        RiscVInstruction::L { width, dest, src } => vec![ArmInstruction::Ldr {
            width: map_width(&width),
            dest: map_register(dest, &width),
            src: map_val(src, &width),
        }],
        RiscVInstruction::Directive { name, operands } => {
            let arm_operands = operands.replace("@", "%");
            vec![ArmInstruction::Directive {
                name,
                operands: arm_operands,
            }]
        }
        RiscVInstruction::Label { name } => vec![ArmInstruction::Label { name }],
        RiscVInstruction::Mv { dest, src } => {
            let width = RiscVWidth::Double;
            vec![ArmInstruction::Add {
                dest: map_register(dest, &width),
                arg1: map_register(src, &width),
                arg2: ArmVal::Imm(0),
            }]
        }
        RiscVInstruction::Mvi { dest, imm } => {
            let width = RiscVWidth::Double;
            vec![ArmInstruction::Mov {
                width: map_width(&width),
                dest: map_register(dest, &width),
                src: ArmVal::Imm(imm),
            }]
        }
        RiscVInstruction::Add {
            width,
            dest,
            arg1,
            arg2,
        } => match width {
            RiscVWidth::Word => vec![ArmInstruction::Add {
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
            }],
            RiscVWidth::Double => sorry!(),
        },
        RiscVInstruction::SextW { dest, src } => vec![ArmInstruction::Sxtw {
            dest: ArmRegister {
                width: ArmWidth::Double,
                name: map_register_name(dest),
            },
            src: ArmRegister {
                width: ArmWidth::Word,
                name: map_register_name(src),
            },
        }],
        RiscVInstruction::Jr { target } => vec![ArmInstruction::Blr {
            target: map_register_name(target),
        }],
        RiscVInstruction::Li { dest, imm } => {
            if imm > 4095 || imm < 0 {
                panic!("Li with imm out of range");
            }

            let width = RiscVWidth::Double;
            vec![ArmInstruction::Mov {
                width: map_width(&width),
                dest: map_register(dest, &width),
                src: ArmVal::Imm(imm),
            }]
            // ArmInstruction::Add {
            //     dest: map_register(dest, &RiscVWidth::Double),
            //     arg1: ArmRegister {
            //         width: ArmWidth::Double,
            //         name: ArmRegisterName::Zero,
            //     },
            //     arg2: ArmVal::Imm(imm),
            // }
        }
        RiscVInstruction::Addl { dest, src, label } => {
            let width = RiscVWidth::Double;
            vec![ArmInstruction::Add {
                dest: map_register(dest, &width),
                arg1: map_register(src, &width),
                arg2: map_val(label, &width),
            }]
        }
        RiscVInstruction::Lui { dest, src } => {
            // only used to load upper bits or adrp in arm
            let width = RiscVWidth::Double;
            vec![ArmInstruction::Adrp {
                dest: map_register(dest, &width),
                label: map_val(src, &width),
            }]
        }
        RiscVInstruction::Call { label } => {
            let width = RiscVWidth::Double;
            vec![ArmInstruction::Bl {
                target: map_val(label, &width),
            }]
        }
        RiscVInstruction::ECall => {
            let syscall_num_reg = ArmRegister {
                width: ArmWidth::Double,
                name: ArmRegisterName::X8,
            };
            vec![
                // ArmInstruction::Cmp(syscall_num_reg, ArmVal::Imm(RISCV_WRITE)),                                                      // if (x8 == RISCV_WRITE) {
                // ArmInstruction::Bne("else"),
                // ArmInstruction::Mov { width: ArmWidth::Double, dest: x8, src: ArmVal::Imm(SYS_WRITE) }, // x8 = ARM_WRITE;
                // ArmInstruction::B("done"),
                // ArmInstruction::Label("else"),                                                   // } else {
                // ArmInstruction::Mov { width: ArmWidth::Double, dest: x8, src: ArmVal::Imm(__) }, //        x8 = ARM_EXIT
                //                                                                                  // }
                // ArmInstruction::Label("done"),
                ArmInstruction::Svc { id: 0 },
            ]
        }
        RiscVInstruction::Verbatim { text } => vec![ArmInstruction::Verbatim { text }],
    }
}

fn map_register(riscv_reg: RiscVRegister, riscv_width: &RiscVWidth) -> ArmRegister {
    ArmRegister {
        width: map_width(riscv_width),
        name: map_register_name(riscv_reg),
    }
}

/// Semantic meaning of registers
/// https://riscv.org/wp-content/uploads/2024/12/riscv-calling.pdf#page=3
fn map_register_name(riscv_reg: RiscVRegister) -> ArmRegisterName {
    match riscv_reg {
        RiscVRegister::X0 => ArmRegisterName::Zero,
        RiscVRegister::RA => ArmRegisterName::Lr,
        RiscVRegister::SP => ArmRegisterName::Sp,
        RiscVRegister::GP => ArmRegisterName::X12,
        RiscVRegister::TP => ArmRegisterName::X14,
        RiscVRegister::T0 => ArmRegisterName::X9,
        RiscVRegister::T1 => ArmRegisterName::X10,
        RiscVRegister::T2 => ArmRegisterName::X11,
        // skipped X5
        //         RiscVRegister::S1 => ArmRegisterName::X6,
        //         RiscVRegister::A0 => ArmRegisterName::X0,
        //         RiscVRegister::A1 => ArmRegisterName::X1,
        //         RiscVRegister::A2 => ArmRegisterName::X2,
        //         RiscVRegister::A3 => ArmRegisterName::X3,
        //         RiscVRegister::A4 => ArmRegisterName::X4,
        //         RiscVRegister::A5 => ArmRegisterName::X5,
        //         RiscVRegister::A6 => ArmRegisterName::X6,
        //         RiscVRegister::A7 => ArmRegisterName::X7,
        RiscVRegister::S1 => ArmRegisterName::X13,
        RiscVRegister::A0 => ArmRegisterName::X0, // return value/syscall arg 0
        RiscVRegister::A1 => ArmRegisterName::X1, // syscall arg 1
        RiscVRegister::A2 => ArmRegisterName::X2, // syscall arg 2
        RiscVRegister::A3 => ArmRegisterName::X3, // syscall arg 3
        RiscVRegister::A4 => ArmRegisterName::X4, // syscall arg 4
        RiscVRegister::A5 => ArmRegisterName::X5, // syscall arg 5
        RiscVRegister::A6 => ArmRegisterName::X6, // syscall arg 6
        RiscVRegister::A7 => ArmRegisterName::X8, // syscall number
        RiscVRegister::S2 => ArmRegisterName::X15,
        RiscVRegister::S3 => ArmRegisterName::X16,
        RiscVRegister::S4 => ArmRegisterName::X17,
        RiscVRegister::S5 => ArmRegisterName::X18,
        RiscVRegister::S6 => ArmRegisterName::X19,
        RiscVRegister::S7 => ArmRegisterName::X20,
        RiscVRegister::S8 => ArmRegisterName::X21,
        RiscVRegister::S9 => ArmRegisterName::X22,
        RiscVRegister::S10 => ArmRegisterName::X23,
        RiscVRegister::S11 => ArmRegisterName::X24,
        RiscVRegister::T3 => ArmRegisterName::X25,
        RiscVRegister::T4 => ArmRegisterName::X26,
        RiscVRegister::T5 => ArmRegisterName::X27,
        RiscVRegister::T6 => ArmRegisterName::X28,
        RiscVRegister::S0FP => ArmRegisterName::X29,
    }
}

fn map_val(riscv_val: RiscVVal, riscv_width: &RiscVWidth) -> ArmVal {
    match riscv_val {
        RiscVVal::RiscVRegister(riscv_reg) => ArmVal::Reg(map_register(riscv_reg, riscv_width)),
        RiscVVal::Immediate(imm) => ArmVal::Imm(imm),
        RiscVVal::Offset { register, offset } => {
            ArmVal::RegOffset(map_register(register, riscv_width), offset)
        }
        RiscVVal::LabelOffset { label, offset } => ArmVal::LabelOffset(label, offset),
    }
}

fn map_width(riscv_width: &RiscVWidth) -> ArmWidth {
    // todo!()
    // FIXME: do real implementation
    match riscv_width {
        RiscVWidth::Double => ArmWidth::Double,
        RiscVWidth::Word => ArmWidth::Word,
    }
}

// Translate every instruction 1:1
pub fn translate_instrs(riscv_instrs: Vec<RiscVInstruction>) -> Vec<ArmInstruction> {
    riscv_instrs
        .into_iter()
        .map(translate)
        .fold(vec![], |mut acc, x| {
            acc.extend(x);
            acc
        })
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
