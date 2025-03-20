buf:
    .string "Hello world!"

.section .text
.globl _start

_start:
    # while i < 1000
    li a3, 1000
.loop:
    addi a3, a3, -1
    ble a3, x0, .end


    # write(stdout, buf, len)

    # syscall number
    li a7,64
    # arg 2
    li a2,13
    # arg 1
    lui a0,%hi(buf)
    addi a1,a0,%lo(buf)
    # arg 0
    li a0,1
    # syscall
    ecall
    j .loop

.end:
    # exit(0)

    # syscall number
    li a7,93
    # arg 0
    li a0,0
    # syscall
    ecall


