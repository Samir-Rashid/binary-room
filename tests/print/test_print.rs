#[cfg(test)]
mod tests {
    use binary_room::instruction::*;
    use binary_room::translate::*;
    use binary_room::utils;
    use binary_room::utils::translate_to_file;
    use binary_room::utils::ARM_START;

    const buf: &str = r#"
buf:
    .string "hello world\n"
"#;

    #[test]
    fn test_print_translate() {
        let riscv_asm: Vec<RiscVInstruction> = vec![
            RiscVInstruction::Verbatim {
                text: buf.to_string(),
            },
            RiscVInstruction::Verbatim {
                text: ARM_START.to_string(),
            },
            // While i < 1000
            RiscVInstruction::Li {
                dest: RiscVRegister::A3,
                imm: 1000,
            },
            RiscVInstruction::Label {
                name: ".loop".to_string(),
            },
            RiscVInstruction::Addi {
                dest: RiscVRegister::A3,
                src: RiscVRegister::A3,
                imm: -1,
            },
            RiscVInstruction::Ble {
                arg1: RiscVRegister::A3,
                arg2: RiscVRegister::X0,
                target: RiscVVal::LabelOffset {
                    label: ".end".to_string(),
                    offset: 0,
                },
            },
            // write syscall
            RiscVInstruction::Li {
                dest: RiscVRegister::A7,
                imm: 64,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A2,
                imm: 14,
            },
            RiscVInstruction::Lui {
                dest: RiscVRegister::A0,
                src: RiscVVal::LabelOffset {
                    label: "buf".to_string(),
                    offset: 9998,
                },
            },
            RiscVInstruction::Addl {
                dest: RiscVRegister::A1,
                src: RiscVRegister::A0,
                label: RiscVVal::LabelOffset {
                    label: "buf".to_string(),
                    offset: 9999,
                },
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A0,
                imm: 1,
            },
            RiscVInstruction::ECall,
            RiscVInstruction::J {
                target: RiscVVal::LabelOffset {
                    label: ".loop".to_string(),
                    offset: 0,
                },
            },
            RiscVInstruction::Label {
                name: ".end".to_string(),
            },
            // exit syscall
            RiscVInstruction::Li {
                dest: RiscVRegister::A7,
                imm: 93,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A0,
                imm: 0,
            },
            RiscVInstruction::ECall,
        ];

        translate_to_file(riscv_asm, "./tests/print/print.arm.s".to_string());
    }
}
