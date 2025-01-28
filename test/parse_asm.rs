#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(instructions[0], Instruction::Addi);
        assert_eq!(instructions[1], Instruction::Sd);
        assert_eq!(instructions[2], Instruction::Ld);
        assert_eq!(instructions[3], Instruction::Addi);
        assert_eq!(instructions[4], Instruction::Li);
        assert_eq!(instructions[5], Instruction::Sw);
        assert_eq!(instructions[6], Instruction::Li);
        assert_eq!(instructions[7], Instruction::Sw);
        assert_eq!(instructions[8], Instruction::Lw);
        assert_eq!(instructions[9], Instruction::Mv);
        assert_eq!(instructions[10], Instruction::Lw);
        assert_eq!(instructions[11], Instruction::Addw);
        assert_eq!(instructions[12], Instruction::SextW);
        assert_eq!(instructions[13], Instruction::Mv);
        assert_eq!(instructions[14], Instruction::Ld);
        assert_eq!(instructions[15], Instruction::Ld);
        assert_eq!(instructions[16], Instruction::Addi);
        assert_eq!(instructions[17], Instruction::Jr);
    }
}
