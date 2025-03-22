use std::fs;

use crate::{instruction::RiscVInstruction, translate::translate_instrs};

/// Loop main() 10,000 times. Uses a3.
pub const RISCV_LOOP_START: &str = r#"
.text

.global _start
.global _main

.balign 4 # not sure if these are needed for RISC-V
_start:
   # while i < 10,000
    li a3, 10000
.loop:
    addi a3, a3, -1
    ble a3, x0, .end

    # main()
    jal ra, main

    # while loop
    j .loop
.end:
    # exit(0)
    li a7,93
    ecall

.balign 4
_main:
main:
"#;

/// Loop main() 10,000 times. Uses x3
pub const ARM_LOOP_START: &str = r#"
.text

.global _start
.global _main

.balign 4
_start:
# i = 10,000
mov x3, #10000
# while i > 0
.loop:
sub x3, x3, 1

cmp x3, xzr
ble .end

# main()
bl main

b .loop
.end:
mov x8, #93
svc #0

.balign 4
_main:
main:
"#;

pub const ARM_START: &str = r#"
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
"#;

pub const START_NO_MAIN: &str = r#"
.text

.global _start
.global _main

.balign 4
_start:
bl main
mov x8, #93
svc #0
"#;

/// Assembler directives for main only, used when another
/// function defined before main
pub const START_MAIN: &str = r#"
.balign 4
_main:
main:
"#;

pub fn translate_to_file(instrs: Vec<RiscVInstruction>, path: String) {
    let arm_instrs = translate_instrs(instrs);
    let mut contents = String::new();
    for instr in arm_instrs {
        let x: String = instr.into();
        contents.push_str(&x);
        contents.push_str("\n");
    }
    fs::write(&path, contents).expect("Unable to write file");
    println!("Saved ARM assembly to {}", path);
}
