
buf:
    .string "hello world\n"


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

mov x3, 1000
.loop:
sub x3, x3, 1
cmp x3, xzr
ble .end
mov x8, 64
mov x2, 14
adrp x0, buf
add x1, x0, :lo12:buf
mov x0, 1
svc 0
b .loop
.end:
mov x8, 93
mov x0, 0
svc 0
