#[cfg(test)]
mod tests {
    use binary_room::translate::binary_translate;

    #[test]
    fn test_binary_translate() {
        let riscv_asm = "
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
        let translated_asm = binary_translate(riscv_asm);
        let expected_output = "
            Addi
            Sd
            Ld
            Addi
            Li
            Sw
            Li
            Sw
            Lw
            Mv
            Lw
            Addw
            SextW
            Mv
            Ld
            Ld
            Addi
            Jr
        ";
        assert_eq!(translated_asm, expected_output);
    }
}
