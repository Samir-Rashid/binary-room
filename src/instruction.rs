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
    /// Store 64-bit, values from register rs2 to memory.
    ///
    /// `M[x[rs1] + sext(offset)] = x[rs2][63:0]`
    #[strum(serialize = "sd")]
    Sd {
        dest: RiscVRegister,
        src: RiscVRegister,
    },
    /// Loads a 64-bit value from memory into register rd for RV64I.
    ///
    /// `x[rd] = M[x[rs1] + sext(offset)][63:0]`
    #[strum(serialize = "ld")]
    Ld { dest: RiscVRegister, src: RiscVVal },
    /// Loads a 32-bit value from memory and sign-extends this to XLEN bits
    /// before storing it in register rd.
    ///
    /// `x[rd] = sext(M[x[rs1] + sext(offset)][31:0])`
    #[strum(serialize = "lw")]
    Lw { dest: RiscVRegister, src: RiscVVal },
    /// Store 32-bit, values from the low bits of register rs2 to memory.
    ///
    /// `M[x[rs1] + sext(offset)] = x[rs2][31:0]`
    #[strum(serialize = "sw")]
    Sw { dest: RiscVRegister, src: RiscVVal },
    // Copy register
    // `mv rd, rs1` expands to `addi rd, rs, 0`
    #[strum(serialize = "mv")]
    Mv {
        dest: RiscVRegister,
        src: RiscVRegister,
    },
    #[strum(serialize = "addw")]
    Addw,
    /// Sign extend Word
    ///
    /// psuedo instruction which translates to `addiw rd, rs, 0`
    #[strum(serialize = "sext.w")]
    SextW {
        dest: RiscVRegister,
        src: RiscVRegister,
    },
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
    Li { imm: i32 },
}

#[derive(Debug)]
pub enum ArmVal {
    Reg(ArmRegister),
    Imm(i32),
    RegOffset(ArmRegister, i32),
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
    #[strum(serialize = "ldr")]
    Ldr,
    #[strum(serialize = "mov")]
    Mov,
    #[strum(serialize = "ret")]
    Ret,
    /// Str [r2 + offset] = r1
    #[strum(serialize = "str")]
    Str {
        src: ArmRegister,
        dst: ArmVal,
    }
    /// Sub Sub Rd := Rn - Op2
    #[strum(serialize = "sub")]
    Sub {
        dest: ArmRegister,
        arg1: ArmRegister,
        arg2: ArmVal,
    },
}

#[derive(Debug)]
pub enum RiscVVal {
    RiscVRegister,
    Immediate(i32),
    /// This is for arguments to opcodes which have an offset
    Offset {
        register: Box<RiscVRegister>,
        offset: i32,
    },
}

/// RISC-V Registers
/// https://msyksphinz-self.github.io/riscv-isadoc/html/regs.html
#[derive(Debug, EnumString)]
pub enum RiscVRegister {
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
                RiscVInstruction::from_str(parts[0]).ok()
            }
        })
        .collect()
}
