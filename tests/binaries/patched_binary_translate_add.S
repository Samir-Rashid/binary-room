.text

.global _start

_start:
bl main
mov x8, #93
svc #0

main:
sub sp, sp, 32
str lr, [sp, 24]
str fp, [sp, 16]
add fp, sp, 32
mov x12, 3
str w12, [fp, -20]
mov x12, 4
str w12, [fp, -24]
ldr w12, [fp, -20]
add x11, x12, 0
ldr w12, [fp, -24]
add w12, w11, w12
sxtw x12, w12
add x0, x12, 0
ldr lr, [sp, 24]
ldr fp, [sp, 16]
add sp, sp, 32
ret

