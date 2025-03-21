#[cfg(test)]
mod tests {
    use binary_room::instruction::*;
    use binary_room::utils::translate_to_file;
    use binary_room::utils::START_MAIN;
    use binary_room::utils::START_NO_MAIN;
    const N: i32 = 4093;

    #[test]
    fn test_binary_translate() {
        let riscv_asm: Vec<RiscVInstruction> = vec![
            RiscVInstruction::Verbatim {
                text: START_NO_MAIN.to_string(),
            },
            RiscVInstruction::Label {
                name: "is_prime".to_string(),
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: -48,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::S0FP,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 40,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::S0FP,
                src: RiscVRegister::SP,
                imm: 48,
            },
            RiscVInstruction::Mv {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A0,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -36,
                },
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -36,
                },
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A4,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 1,
            },
            RiscVInstruction::Bgt {
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
                target: RiscVVal::LabelOffset {
                    label: ".L2".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 0,
            },
            RiscVInstruction::J {
                target: RiscVVal::LabelOffset {
                    label: ".L3".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Label {
                name: ".L2".to_string(),
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
                    label: ".L4".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Label {
                name: ".L8".to_string(),
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -36,
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
                    offset: -20,
                },
            },
            RiscVInstruction::Sub {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -24,
                },
            },
            RiscVInstruction::J {
                target: RiscVVal::LabelOffset {
                    label: ".L5".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Label {
                name: ".L6".to_string(),
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -24,
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
                    offset: -20,
                },
            },
            RiscVInstruction::Sub {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Word,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -24,
                },
            },
            RiscVInstruction::Label {
                name: ".L5".to_string(),
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -24,
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
                    offset: -20,
                },
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A4,
                src: RiscVRegister::A4,
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Bge {
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
                target: RiscVVal::LabelOffset {
                    label: ".L6".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::L {
                width: RiscVWidth::Word,
                dest: RiscVRegister::A5,
                src: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -24,
                },
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Bne {
                arg1: RiscVRegister::A5,
                arg2: RiscVRegister::X0,
                target: RiscVVal::LabelOffset {
                    label: ".L7".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 0,
            },
            RiscVInstruction::J {
                target: RiscVVal::LabelOffset {
                    label: ".L3".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Label {
                name: ".L7".to_string(),
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
                imm: 1,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::A5,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::S0FP,
                    offset: -20,
                },
            },
            RiscVInstruction::Label {
                name: ".L4".to_string(),
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
                    offset: -36,
                },
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A4,
                src: RiscVRegister::A4,
            },
            RiscVInstruction::SextW {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A5,
            },
            RiscVInstruction::Blt {
                arg1: RiscVRegister::A4,
                arg2: RiscVRegister::A5,
                target: RiscVVal::LabelOffset {
                    label: ".L8".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A5,
                imm: 1,
            },
            RiscVInstruction::Label {
                name: ".L3".to_string(),
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
                    offset: 40,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: 48,
            },
            RiscVInstruction::Jr {
                target: RiscVRegister::RA,
            },
            RiscVInstruction::Verbatim {
                text: START_MAIN.to_string(),
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: -16,
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::RA,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 8,
                },
            },
            RiscVInstruction::S {
                width: RiscVWidth::Double,
                src: RiscVRegister::S0FP,
                dest: RiscVVal::Offset {
                    register: RiscVRegister::SP,
                    offset: 0,
                },
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::S0FP,
                src: RiscVRegister::SP,
                imm: 16,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A0,
                imm: N,
            },
            RiscVInstruction::Call {
                label: RiscVVal::LabelOffset {
                    label: "is_prime".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Mv {
                dest: RiscVRegister::A5,
                src: RiscVRegister::A0,
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
                    offset: 8,
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
                imm: 16,
            },
            RiscVInstruction::Jr {
                target: RiscVRegister::RA,
            },
        ];

        translate_to_file(riscv_asm, "./tests/prime/prime.arm.s".to_string());
    }
}
