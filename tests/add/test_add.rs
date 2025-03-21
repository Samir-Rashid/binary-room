#[cfg(test)]
mod tests {
    use binary_room::instruction::*;
    use binary_room::translate::*;
    use binary_room::utils;
    use binary_room::utils::translate_to_file;
    use binary_room::utils::ARM_LOOP_START;

    #[test]
    fn test_binary_translate() {
        let riscv_asm: Vec<RiscVInstruction> = vec![
            RiscVInstruction::Verbatim {
                text: ARM_LOOP_START.to_string(),
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: -32,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::RA,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 24,
                },
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::S0FP,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 16,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::S0FP,
                src: RiscVRegister::SP,
                imm: 32,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 3,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 4,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -24,
                },
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Mv {
                dest: RiscVRegister::A4,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -24,
                },
            },
            RiscVInstruction::Add {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Mv {
                dest: RiscVRegister::A0,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::RA,
                src: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 24,
                },
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::S0FP,
                src: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 16,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: 32,
            },
            RiscVInstruction::Jr {
                target: RiscVRegister::RA,
            },
        ];

        translate_to_file(riscv_asm, "./tests/add/add.arm.s".to_string());
    }
}
