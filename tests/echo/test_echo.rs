#[cfg(test)]
mod tests {
    use binary_room::instruction::*;
    use binary_room::utils::translate_to_file;
    use binary_room::utils::ARM_START;

    const BUF: &str = r#"
.buf:
    .string "hello world"
"#;

    #[test]
    fn test_print_translate() {
        let riscv_asm: Vec<RiscVInstruction> = vec![
            // RiscVInstruction::Verbatim { text: buf.to_string() },
            RiscVInstruction::Verbatim {
                text: ARM_START.to_string(),
            },
            // read syscall
            RiscVInstruction::Addi {
                dest: RiscVRegister::SP,
                src: RiscVRegister::SP,
                imm: -32,
            }, // sub stack pointer
            RiscVInstruction::Li {
                dest: RiscVRegister::A7,
                imm: 63,
            }, // read syscall #
            RiscVInstruction::Li {
                dest: RiscVRegister::A2,
                imm: 32,
            }, // read 5 bytes
            RiscVInstruction::Mv {
                dest: RiscVRegister::A1,
                src: RiscVRegister::SP,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A0,
                imm: 0,
            },
            RiscVInstruction::ECall,
            // write syscall
            RiscVInstruction::Li {
                dest: RiscVRegister::A7,
                imm: 64,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A2,
                imm: 14,
            },
            RiscVInstruction::Mv {
                dest: RiscVRegister::A1,
                src: RiscVRegister::SP,
            },
            RiscVInstruction::Li {
                dest: RiscVRegister::A0,
                imm: 1,
            },
            RiscVInstruction::ECall,
            // exit syscall
            RiscVInstruction::Li {
                dest: RiscVRegister::A7,
                imm: 93,
            },
            // RiscVInstruction::Li { dest: RiscVRegister::A0, imm: 0 },
            RiscVInstruction::ECall,
        ];

        translate_to_file(riscv_asm, "./tests/echo/echo.arm.s".to_string());
    }
}
