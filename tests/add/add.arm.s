
.text

.global _start
.global _main

.balign 4
_start:
bl main
mov x8, #93
svc #0

.balign 4
_main:
main:

sub sp, sp, 32
str lr, [sp, 24]
str x29, [sp, 16]
add x29, sp, 32
mov x5, 3
str w5, [x29, -20]
mov x5, 4
str w5, [x29, -24]
ldr w5, [x29, -20]
add x4, x5, 0
ldr w5, [x29, -24]
add w5, w4, w5
sxtw x5, w5
add x0, x5, 0
ldr lr, [sp, 24]
ldr x29, [sp, 16]
add sp, sp, 32
blr lr
