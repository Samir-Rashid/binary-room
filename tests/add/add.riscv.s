.text

.global _start
.global _main

.balign 4 # not sure if these are needed for RISC-V
_start:
    # while i < 10,000
    li a3, 10000
.loop:
    addi a3, a3, -1
    ble a3, x0, .end

    # main()
    jal ra, main

    # while loop
    j .loop
.end:
    # exit(0)
    li a7,93
    ecall
main:
        addi    sp,sp,-32
        sd      ra,24(sp)
        sd      s0,16(sp)
        addi    s0,sp,32
        li      a5,3
        sw      a5,-20(s0)
        li      a5,4
        sw      a5,-24(s0)
        lw      a5,-20(s0)
        mv      a4,a5
        lw      a5,-24(s0)
        addw    a5,a4,a5
        sext.w  a5,a5
        mv      a0,a5
        ld      ra,24(sp)
        ld      s0,16(sp)
        addi    sp,sp,32
        jr      ra
