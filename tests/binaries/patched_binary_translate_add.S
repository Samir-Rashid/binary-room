sub sp, sp, -32
str lr, [sp, 24]
str x5, [sp, 16]
add w5, wsp, 32
mov x12, 3
str w12, [x5, -20]
mov x12, 4
str w12, [x5, -24]
ldr w12, [x5, -20]
add x11, x12, 0
ldr w12, [x5, -24]
add w12, w11, w12
sxtw x12, w12
add x7, x12, 0
ldr lr, [sp, 24]
ldr x5, [sp, 16]
add sp, sp, 32
blr lr
