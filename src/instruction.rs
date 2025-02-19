use std::default;

/// This file defines all the supported ARM and RISC-V instructions we support.
/// We use `strum` to assist in serializing asm files to our [`Instruction`] enum.
///
/// We do not aim for completness of translating every possible instruction,
/// but we do want to thoroughly test for correctness.
///
/// Some relevant references for making enums of instructions
/// https://github.com/lmcad-unicamp/riscv-sbt/blob/93bd48525362d00c6a2d7b320dc9cd9e62bc8fa9/sbt/Instruction.h#L62
/// https://github.com/nbdd0121/r2vm/blob/5118be6b9e757c6fef2f019385873f403c23c548/lib/riscv/src/op.rs#L30
use strum_macros::EnumString;

#[derive(Debug, Default)]
pub enum RiscVWidth {
    Word,
    #[default]
    Double,
}

/// RISC-V Instructions
/// https://msyksphinz-self.github.io/riscv-isadoc/html/rvi.html
///
/// To make a function call in RISC-V you use the `jal` (jump and link)
/// instruction. This would require us ensure that we translate the RISC-V
/// calling convention into ARM. (`https://riscv.org/wp-content/uploads/2024/12/riscv-calling.pdf)
#[derive(Debug, EnumString)]
pub enum RiscVInstruction {
    /// add immediate
    ///
    /// `x[rd] = x[rs1] + sext(immediate)`
    #[strum(serialize = "addi")]
    Addi {
        dest: RiscVRegister,
        src: RiscVRegister,
        imm: i32,
    },
    /// add register
    ///     either add or addw
    ///     (addw is 32 bits on 64 bit riscv)
    ///
    /// `x[rd] = sext((x[rs1] + x[rs2])[31:0])`
    #[strum(serialize = "addw")]
    Add {
        // dest = arg1 + arg2
        width: RiscVWidth,
        dest: RiscVRegister,
        arg1: RiscVRegister,
        arg2: RiscVRegister,
    },
    /// Store values from register rs2 to memory.
    ///
    /// `M[x[rs1] + sext(offset)] = x[rs2]`
    #[strum(serialize = "sd")]
    S {
        width: RiscVWidth,
        src: RiscVRegister,
        dest: RiscVVal,
    },
    /// Loads a value from memory into register rd for RV64I.
    ///
    /// `x[rd] = M[x[rs1] + sext(offset)]`
    #[strum(serialize = "ld")]
    L {
        width: RiscVWidth,
        dest: RiscVRegister,
        src: RiscVVal,
    },
    // Copy register
    // `mv rd, rs1` expands to `addi rd, rs, 0`
    #[strum(serialize = "mv")]
    Mv {
        dest: RiscVRegister,
        src: RiscVRegister,
    },
    /// Sign extend Word
    ///
    /// psuedo instruction which translates to `addiw rd, rs, 0`
    #[strum(serialize = "sext.w")]
    SextW {
        dest: RiscVRegister,
        src: RiscVRegister,
    },
    /// Jump Register
    /// Jump to address and place return address in rd.
    /// jal rd,offset
    ///
    /// Psuedo instruction:
    ///     jr offset => jal x1, offset
    ///
    #[strum(serialize = "jr")]
    Jr { target: RiscVRegister },
    /// Load Immediate
    /// This is a pseudo instruction, so it's not a real instruction
    ///
    /// Assembler Pseudo-instructions
    /// The assembler implements a number of convenience psuedo-instructions
    /// that are formed from instructions in the base ISA, but have implicit
    /// arguments or in some case reversed arguments, that result in distinct
    /// semantics.
    /// https://michaeljclark.github.io/asm.html
    #[strum(serialize = "li")]
    Li { dest: RiscVRegister, imm: i32 },
}

impl Default for RiscVInstruction {
    fn default() -> Self {
        Self::Li {
            dest: RiscVRegister::X0,
            imm: 0,
        }
    }
}

#[derive(Debug)]
pub enum ArmVal {
    Reg(ArmRegister),
    Imm(i32),
    RegOffset(ArmRegister, i32),
}

impl Default for ArmVal {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum ArmWidth {
    Byte,
    SignedByte,
    Half,
    SignedHalf,
    Word,
    Double,
}

impl Default for ArmWidth {
    fn default() -> Self {
        todo!()
    }
}

/// ARM Instructions
/// `https://iitd-plos.github.io/col718/ref/arm-instructionset.pdf#page=3`
#[derive(Debug, EnumString)]
pub enum ArmInstruction {
    /// ADC Add with carry
    ///
    /// `Rd := Rn + Op2 + Carry`
    #[strum(serialize = "adc")]
    Adc,
    /// ADD Add Rd := Rn + Op2
    #[strum(serialize = "add")]
    Add {
        dest: ArmRegister,
        arg1: ArmRegister,
        arg2: ArmVal,
    },
    /// AND AND Rd := Rn AND Op2
    #[strum(serialize = "and")]
    And,
    /// B Branch R15 := address
    #[strum(serialize = "b")]
    B,
    /// BLR Xn
    #[strum(serialize = "blr")]
    Blr { target: ArmRegisterName },
    #[strum(serialize = "ldr")]
    Ldr {
        width: ArmWidth,
        dest: ArmRegister,
        src: ArmVal,
    },
    #[strum(serialize = "mov")]
    Mov,
    #[strum(serialize = "ret")]
    Ret,
    /// Str [r2 + offset] = r1
    #[strum(serialize = "str")]
    Str {
        width: ArmWidth,
        src: ArmRegister,
        dest: ArmVal,
    },
    /// Sub Sub Rd := Rn - Op2
    #[strum(serialize = "sub")]
    Sub {
        dest: ArmRegister,
        arg1: ArmRegister,
        arg2: ArmVal,
    },
    /// sign extend to word
    #[strum(serialize = "sxtw")]
    Sxtw { dest: ArmRegister, src: ArmRegister },
}

impl Default for ArmInstruction {
    fn default() -> Self {
        ArmInstruction::B
    }
}

#[derive(Debug)]
pub enum RiscVVal {
    RiscVRegister(RiscVRegister),
    Immediate(i32),
    /// This is for arguments to opcodes which have an offset
    Offset {
        register: RiscVRegister,
        offset: i32,
    },
}

impl Default for RiscVVal {
    fn default() -> Self {
        Self::Immediate(0)
    }
}

/// RISC-V Registers
/// https://msyksphinz-self.github.io/riscv-isadoc/html/regs.html
#[derive(Debug, EnumString, Default)]
pub enum RiscVRegister {
    #[default]
    #[strum(serialize = "x0")]
    /// Hard-wired zero
    X0,
    #[strum(serialize = "ra")]
    /// Return address
    RA,
    #[strum(serialize = "sp")]
    /// Stack pointer
    SP,
    #[strum(serialize = "gp")]
    /// Global pointer
    GP,
    #[strum(serialize = "tp")]
    /// Thread pointer
    TP,
    #[strum(serialize = "t0")]
    /// Temporary/alternate link register
    T0,
    #[strum(serialize = "t1")]
    /// Temporaries
    T1,
    #[strum(serialize = "t2")]
    /// Temporaries
    T2,
    #[strum(serialize = "s0", serialize = "fp")]
    /// Saved register/frame pointer R29
    S0FP,
    #[strum(serialize = "s1")]
    /// Saved registers
    S1,
    #[strum(serialize = "a0")]
    /// Function arguments/return values
    A0,
    #[strum(serialize = "a1")]
    /// Function arguments/return values
    A1,
    #[strum(serialize = "a2")]
    /// Function arguments
    A2,
    #[strum(serialize = "a3")]
    /// Function arguments
    A3,
    #[strum(serialize = "a4")]
    /// Function arguments
    A4,
    #[strum(serialize = "a5")]
    /// Function arguments
    A5,
    #[strum(serialize = "a6")]
    /// Function arguments
    A6,
    #[strum(serialize = "a7")]
    /// Function arguments
    A7,
    #[strum(serialize = "s2")]
    /// Saved registers
    S2,
    #[strum(serialize = "s3")]
    /// Saved registers
    S3,
    #[strum(serialize = "s4")]
    /// Saved registers
    S4,
    #[strum(serialize = "s5")]
    /// Saved registers
    S5,
    #[strum(serialize = "s6")]
    /// Saved registers
    S6,
    #[strum(serialize = "s7")]
    /// Saved registers
    S7,
    #[strum(serialize = "s8")]
    /// Saved registers
    S8,
    #[strum(serialize = "s9")]
    /// Saved registers
    S9,
    #[strum(serialize = "s10")]
    /// Saved registers
    S10,
    #[strum(serialize = "s11")]
    /// Saved registers
    S11,
    #[strum(serialize = "t3")]
    /// Temporaries
    T3,
    #[strum(serialize = "t4")]
    /// Temporaries
    T4,
    #[strum(serialize = "t5")]
    /// Temporaries
    T5,
    #[strum(serialize = "t6")]
    /// Temporaries
    T6,
}

#[derive(Debug)]
pub struct ArmRegister {
    pub width: ArmWidth,
    pub name: ArmRegisterName,
}

impl Default for ArmRegister {
    fn default() -> Self {
        todo!()
    }
}

/// ARM Registers
/// https://developer.arm.com/documentation/dui0056/d/using-the-procedure-call-standard/register-roles-and-names/register-names
/// Image of instructions https://duetorun.com/blog/arm/images/AArch64-registers.png
///   - https://duetorun.com/blog/20230601/a64-regs/#user_program_registers
#[derive(Debug, EnumString)]
pub enum ArmRegisterName {
    #[strum(serialize = "wzr", serialize = "xzr")]
    /// Zero register. Hardware special.
    Zero,
    #[strum(serialize = "pc")]
    /// Program counter. Hardware special register.
    Pc,
    #[strum(serialize = "sp")]
    /// Stack pointer. Hardware special register.
    Sp,
    #[strum(serialize = "lr")]
    /// Link register. X30. Hardware special register.
    Lr,
    // Parameter passing and/or scratch registers (volatile)
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    // Caller-Saved scratch registers (volatile)
    /// XR
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    /// IP0
    X16,
    /// IP1
    X17,
    /// PR
    X18,
    // Caller-Saved registers (non-volatile)
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    /// FP
    X29,
}

impl Default for ArmRegisterName {
    fn default() -> Self {
        todo!()
    }
}

/// Parse a text file into our enum.
pub fn parse_asm(asm: &str) -> Vec<RiscVInstruction> {
    asm.lines()
        .filter_map(|line| {
            // TODO (Samir): Not sure that this will handle assembly labels
            // We probably need to construct a map for those to find the
            // original instruction they map to.
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                None
            } else {
                // RiscVInstruction::from_str(parts[0]).ok()
                todo!()
            }
        })
        .collect()
}
