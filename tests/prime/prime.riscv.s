# int is_prime(int n) {
#     if (n <= 1) {
#         return 0;
#     }
#     for (int i = 2; i < n; i++) {
#         int temp = n - i;
#         while (temp >= i) {
#             temp = temp - i;
#         }
#         if (temp == 0) {
#             // i divides n
#             return 0;
#         }
#     }
#     return 1;
# }
# int main(void) {
# 	return is_prime(4093);
# }

is_prime: # is_prime(int):
addi    sp,sp,-48
sd      s0,40(sp)
addi    s0,sp,48
mv      a5,a0
sw      a5,-36(s0)
lw      a5,-36(s0)
sext.w  a4,a5
li      a5,1
bgt     a4,a5,.L2
li      a5,0
j       .L3
.L2:
li      a5,2
sw      a5,-20(s0)
j       .L4
.L8:
lw      a5,-36(s0)
mv      a4,a5
lw      a5,-20(s0)
subw    a5,a4,a5
sw      a5,-24(s0)
j       .L5
.L6:
lw      a5,-24(s0)
mv      a4,a5
lw      a5,-20(s0)
subw    a5,a4,a5
sw      a5,-24(s0)
.L5:
lw      a5,-24(s0)
mv      a4,a5
lw      a5,-20(s0)
sext.w  a4,a4
sext.w  a5,a5
bge     a4,a5,.L6
lw      a5,-24(s0)
sext.w  a5,a5
bne     a5,zero,.L7
li      a5,0
j       .L3
.L7:
lw      a5,-20(s0)
addiw   a5,a5,1
sw      a5,-20(s0)
.L4:
lw      a5,-20(s0)
mv      a4,a5
lw      a5,-36(s0)
sext.w  a4,a4
sext.w  a5,a5
blt     a4,a5,.L8
li      a5,1
.L3:
mv      a0,a5
ld      s0,40(sp)
addi    sp,sp,48
jr      ra
main:
addi    sp,sp,-16
sd      ra,8(sp)
sd      s0,0(sp)
addi    s0,sp,16
li      a0,4093
call    is_prime
mv      a5,a0
nop
mv      a0,a5
ld      ra,8(sp)
ld      s0,0(sp)
addi    sp,sp,16
jr      ra
