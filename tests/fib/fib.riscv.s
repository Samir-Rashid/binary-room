; int main() {
;     int i;
;     int a[10] = {};
;     a[1] = 1;
;     for (i = 2; i < 10; i++) {
;         a[i] = a[i-1] + a[i-2];
;     }
;     return a[9];
; }

main:
addi    sp,sp,-64
sd      s0,56(sp)
addi    s0,sp,64
sd      zero,-64(s0)
sd      zero,-56(s0)
sd      zero,-48(s0)
sd      zero,-40(s0)
sd      zero,-32(s0)
li      a5,1
sw      a5,-60(s0)
li      a5,2
sw      a5,-20(s0)
j       .L2
.L3:
lw      a5,-20(s0)
addiw   a5,a5,-1
sext.w  a5,a5
slli    a5,a5,2
addi    a5,a5,-16
add     a5,a5,s0
lw      a4,-48(a5)
lw      a5,-20(s0)
addiw   a5,a5,-2
sext.w  a5,a5
slli    a5,a5,2
addi    a5,a5,-16
add     a5,a5,s0
lw      a5,-48(a5)
addw    a5,a4,a5
sext.w  a4,a5
lw      a5,-20(s0)
slli    a5,a5,2
addi    a5,a5,-16
add     a5,a5,s0
sw      a4,-48(a5)
lw      a5,-20(s0)
addiw   a5,a5,1
sw      a5,-20(s0)
.L2:
lw      a5,-20(s0)
sext.w  a4,a5
li      a5,9
ble     a4,a5,.L3
lw      a5,-28(s0)
mv      a0,a5
ld      s0,56(sp)
addi    sp,sp,64
jr      ra