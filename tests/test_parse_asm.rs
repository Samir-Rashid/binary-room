#[cfg(test)]
mod tests {
    use binary_room::instruction::parse_asm;

    #[test]
    fn test_parse_asm() {
        let asm = "
            addi sp,sp,-32
            sd ra,24(sp)
            ld s0,16(sp)
            addi s0,sp,32
            li a5,3
            sw a5,-20(s0)
            li a5,4
            sw a5,-24(s0)
            lw a5,-20(s0)
            mv a4,a5
            lw a5,-24(s0)
            addw a5,a4,a5
            sext.w a5,a5
            mv a0,a5
            ld ra,24(sp)
            ld s0,16(sp)
            addi sp,sp,32
            jr ra
        ";
        let instructions = parse_asm(asm);
        assert_eq!(instructions.len(), 17);
        assert_eq!(instructions[0], RiscVInstruction::Addi);
        assert_eq!(instructions[1], RiscVInstruction::Sd);
        assert_eq!(instructions[2], RiscVInstruction::Ld);
        assert_eq!(instructions[3], RiscVInstruction::Addi);
        assert_eq!(instructions[4], RiscVInstruction::Li);
        assert_eq!(instructions[5], RiscVInstruction::Sw);
        assert_eq!(instructions[6], RiscVInstruction::Li);
        assert_eq!(instructions[7], RiscVInstruction::Sw);
        assert_eq!(instructions[8], RiscVInstruction::Lw);
        assert_eq!(instructions[9], RiscVInstruction::Mv);
        assert_eq!(instructions[10], RiscVInstruction::Lw);
        assert_eq!(instructions[11], RiscVInstruction::Addw);
        assert_eq!(instructions[12], RiscVInstruction::SextW);
        assert_eq!(instructions[13], RiscVInstruction::Mv);
        assert_eq!(instructions[14], RiscVInstruction::Ld);
        assert_eq!(instructions[15], RiscVInstruction::Ld);
        assert_eq!(instructions[16], RiscVInstruction::Addi);
        assert_eq!(instructions[17], RiscVInstruction::Jr);
    }
}
