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

/// RISC-V Instructions
///
/// To make a function call in RISC-V you use the `jal` (jump and link)
/// instruction. This would require us ensure that we translate the RISC-V
/// calling convention into ARM. (`https://riscv.org/wp-content/uploads/2024/12/riscv-calling.pdf)
#[derive(Debug, EnumString)]
pub enum RiscVInstruction {
    #[strum(serialize = "addi")]
    Addi {
        dest: RiscVRegister,
        src: RiscVRegister,
        imm: i32,
    },
    #[strum(serialize = "sd")]
    Sd,
    #[strum(serialize = "ld")]
    Ld,
    #[strum(serialize = "lw")]
    Lw,
    #[strum(serialize = "sw")]
    Sw,
    #[strum(serialize = "lw")]
    Lw,
    #[strum(serialize = "mv")]
    Mv,
    #[strum(serialize = "addw")]
    Addw,
    #[strum(serialize = "sext.w")]
    SextW,
    #[strum(serialize = "jr")]
    Jr,
    #[strum(serialize = "li")]
    Li,
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
        arg2: ArmRegister,
    },
    /// AND AND Rd := Rn AND Op2
    #[strum(serialize = "and")]
    And,
    /// B Branch R15 := address
    #[strum(serialize = "b")]
    B,
}

/// RISC-V Registers
/// https://msyksphinz-self.github.io/riscv-isadoc/html/regs.html
#[derive(Debug, EnumString)]
pub enum RiscVRegister {
    /// This is for arguments to opcodes which have an offset
    /// I'm not sure how to make strum happy, so this doesn't auto parse.
    #[strum(disabled)]
    Offset {
        register: Box<RiscVRegister>,
        offset: usize,
    },
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
    /// Saved register/frame pointer
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

/// ARM Registers
/// https://developer.arm.com/documentation/dui0056/d/using-the-procedure-call-standard/register-roles-and-names/register-names
#[derive(Debug, EnumString)]
pub enum ArmRegister {
    #[strum(serialize = "pc")]
    /// Program counter.
    Pc,
    #[strum(serialize = "lr")]
    /// Link register.
    Lr,
    #[strum(serialize = "sp")]
    /// Stack pointer.
    Sp,
    #[strum(serialize = "ip")]
    /// Intra-procedure-call scratch register.
    Ip,
    #[strum(serialize = "v8")]
    /// ARM-state variable register 8.
    V8,
    #[strum(serialize = "sl")]
    /// ARM-state variable register 7. Stack limit pointer in stack-checked variants.
    Sl,
    #[strum(serialize = "sb")]
    /// ARM-state variable register 6. Static base in RWPI variants.
    Sb,
    #[strum(serialize = "v5")]
    /// ARM-state variable register 5.
    V5,
    #[strum(serialize = "v4")]
    /// Variable register 4.
    V4,
    #[strum(serialize = "v3")]
    /// Variable register 3.
    V3,
    #[strum(serialize = "v2")]
    /// Variable register 2.
    V2,
    #[strum(serialize = "v1")]
    /// Variable register 1.
    V1,
    #[strum(serialize = "a4")]
    /// Argument/result/scratch register 4.
    A4,
    #[strum(serialize = "a3")]
    /// Argument/result/scratch register 3.
    A3,
    #[strum(serialize = "a2")]
    /// Argument/result/scratch register 2.
    A2,
    #[strum(serialize = "a1")]
    /// Argument/result/scratch register 1.
    A1,
}
