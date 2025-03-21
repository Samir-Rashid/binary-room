
.text

.global _start
.global _main

.balign 4
_start:
bl main
mov x8, #93
svc #0

is_prime:
sub sp, sp, 48
str x29, [sp, 40]
add x29, sp, 48
add x5, x0, 0
str w5, [x29, -36]
ldr w5, [x29, -36]
sxtw x4, w5
mov x5, 1
cmp x4, x5
bgt .L2
mov x5, 0
b .L3
.L2:
mov x5, 2
str w5, [x29, -20]
b .L4
.L8:
ldr w5, [x29, -36]
add x4, x5, 0
ldr w5, [x29, -20]
sub w5, w4, w5
str w5, [x29, -24]
b .L5
.L6:
ldr w5, [x29, -24]
add x4, x5, 0
ldr w5, [x29, -20]
sub w5, w4, w5
str w5, [x29, -24]
.L5:
ldr w5, [x29, -24]
add x4, x5, 0
ldr w5, [x29, -20]
sxtw x4, w4
sxtw x5, w5
cmp x4, x5
bge .L6
ldr w5, [x29, -24]
sxtw x5, w5
cmp x5, xzr
bne .L7
mov x5, 0
b .L3
.L7:
ldr w5, [x29, -20]
add x5, x5, 1
str x5, [x29, -20]
.L4:
ldr w5, [x29, -20]
add x4, x5, 0
ldr w5, [x29, -36]
sxtw x4, w4
sxtw x5, w5
cmp x4, x5
blt .L8
mov x5, 1
.L3:
add x0, x5, 0
ldr x29, [sp, 40]
add sp, sp, 48
blr lr

.balign 4
_main:
main:

sub sp, sp, 16
str lr, [sp, 8]
str x29, [sp, 0]
add x29, sp, 16
mov x0, 4093
bl is_prime
add x5, x0, 0
add x0, x5, 0
ldr lr, [sp, 8]
ldr x29, [sp, 16]
add sp, sp, 16
blr lr
