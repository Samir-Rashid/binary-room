
.buf:
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

mov x8, 64
mov x2, 14
adrp x0, .buf
add x1, x0, :lo12:.buf
mov x0, 1
svc 0
mov x8, 93
svc 0
