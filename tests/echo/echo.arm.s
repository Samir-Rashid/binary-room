
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
mov x8, 63
mov x2, 32
add x1, sp, 0
mov x0, 0
svc 0
mov x8, 64
mov x2, 14
add x1, sp, 0
mov x0, 1
svc 0
mov x8, 93
svc 0
