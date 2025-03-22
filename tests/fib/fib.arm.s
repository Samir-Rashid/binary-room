
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

sub sp, sp, 64
str x29, [sp, 56]
add x29, sp, 64
str xzr, [x29, -64]
str xzr, [x29, -56]
str xzr, [x29, -48]
str xzr, [x29, -40]
str xzr, [x29, -32]
mov x5, 1
str w5, [x29, -60]
mov x5, 2
str w5, [x29, -20]
b .L2
.L3:
ldr w5, [x29, -20]
sub x5, x5, 1
sxtw x5, w5
lsl x5, x5, 2
sub x5, x5, 16
add x5, x5, x29
ldr x4, [x5, -48]
ldr x5, [x29, -20]
sub x5, x5, 2
sxtw x5, w5
lsl x5, x5, 2
sub x5, x5, 16
add x5, x5, x29
ldr x5, [x5, -48]
add x5, x4, x5
sxtw x4, w5
ldr x5, [x29, -20]
lsl x5, x5, 2
sub x5, x5, 16
add x5, x5, x29
str x4, [x5, -48]
ldr x5, [x29, -20]
add x5, x5, 1
str w5, [x29, -20]
.L2:
ldr x5, [x29, -20]
sxtw x4, w5
mov x5, 9
cmp x4, x5
ble .L3
ldr w5, [x29, -28]
add x0, x5, 0
ldr x29, [sp, 56]
add sp, sp, 64
blr lr
