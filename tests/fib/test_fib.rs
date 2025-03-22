#[cfg(test)]
mod tests {
    use binary_room::instruction::*;
    use binary_room::utils::translate_to_file;
    use binary_room::utils::ARM_START;

    #[test]
    fn test_binary_translate() {
        let riscv_asm: Vec<RiscVInstruction> = vec![
            RiscVInstruction::Verbatim {
                text: ARM_START.to_string(),
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: -64,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::S0FP,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 56,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::S0FP,
                src: RiscVRegister::SP,
                imm: 64,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::X0,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -64,
                },
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::X0,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -56,
                },
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::X0,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -48,
                },
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::X0,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -40,
                },
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::X0,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -32,
                },
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 1,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -60,
                },
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 2,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::J {
                target: RiscVVal::LabelOffset {
                    label: ".L2".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Label {
                name: ".L3".to_string(),
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: -1,
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Slli {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: 2,
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: -16,
            },
            RiscVInstruction::Add {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                arg1: RiscVRegister::A5,
                arg2: RiscVRegister::S0FP,
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A4,
                src: RiscVVal::Offset {
                    register: RiscVRegister::A5,
                    offset: -48,
                },
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: -2,
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Slli {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: 2,
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: -16,
            },
            RiscVInstruction::Add {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                arg1: RiscVRegister::A5,
                arg2: RiscVRegister::S0FP,
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::A5,
                    offset: -48,
                },
            },
            RiscVInstruction::Add {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A4,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Slli {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: 2,
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: -16,
            },
            RiscVInstruction::Add {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                arg1: RiscVRegister::A5,
                arg2: RiscVRegister::S0FP,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::A4,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::A5,
                    offset: -48,
                },
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
                imm: 1,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Label {
                name: ".L2".to_string(),
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A4,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 9,
            },
            RiscVInstruction::Ble {
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
                target: RiscVVal::LabelOffset {
                    label: ".L3".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -28,
                },
            },
            RiscVInstruction::Mv {
                dest: RiscVRegister::A0,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::L {
                width: RiscVWidth::Double,
                dest: RiscVRegister::S0FP,
                src: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 56,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: 64,
            },
            RiscVInstruction::Jr {
                target: RiscVRegister::RA,
            },
        ];

        translate_to_file(riscv_asm, "./tests/fib/fib.arm.s".to_string());
    }
}
