use std::convert::Into;
use std::default;
use std::fmt::{format, write, Display};

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
    /// add label/offset addr (not a real RISC-V instr)
    Addl {
        dest: RiscVRegister,
        src: RiscVRegister,
        label: RiscVVal
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
    /// call label
    #[strum(serialize = "call")]
    Call {
        label: RiscVVal
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
    Directive {
        name: String,
        operands: String
    },
    Label {
        name: String
    },
    #[strum(serialize = "lui")]
    Lui {
        dest: RiscVRegister,
        src: RiscVVal
    },
    // Copy register
    // `mv rd, rs1` expands to `addi rd, rs, 0`
    #[strum(serialize = "mv")]
    Mv {
        dest: RiscVRegister,
        src: RiscVRegister,
    },
    // Copy immediate
    // `mv rd, rs1` expands to `addi rd, rs, 0`
    #[strum(serialize = "mvi")]
    Mvi {
        dest: RiscVRegister,
        imm: i32,
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
    LabelOffset(String, i32)
}

impl Default for ArmVal {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
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
    /// ADRP Rd := page_addr(label)
    #[strum(serialize = "adrp")]
    Adrp {
        dest: ArmRegister,
        label: ArmVal
    },
    /// B Branch R15 := address
    #[strum(serialize = "b")]
    B,
    /// BLR Xn
    #[strum(serialize = "blr")]
    Blr { target: ArmRegisterName },
    /// BL label
    #[strum(serialize = "bl")]
    Bl {target: ArmVal},
    /// label:
    Label { name: String },
    /// .directive operands
    Directive {
        name: String, 
        operands: String
    },
    #[strum(serialize = "ldr")]
    Ldr {
        width: ArmWidth,
        dest: ArmRegister,
        src: ArmVal,
    },
    #[strum(serialize = "mov")]
    Mov {
        width: ArmWidth,
        dest: ArmRegister,
        src: ArmVal
    },
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
    LabelOffset {
        label: String,
        offset: i32
    }
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

#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, EnumString, Copy, Clone)]
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

impl Into<String> for ArmInstruction {
    fn into(self) -> String {
        match self {
            ArmInstruction::Adc => todo!(),
            ArmInstruction::Add { dest, arg1, arg2 } => {
                format!("add {}, {}, {}", dest, arg1, arg2)
            },
            ArmInstruction::And => todo!(),
            ArmInstruction::Adrp { dest, label } => {
                format!("adrp {}, {}", dest, label)
            }
            ArmInstruction::B => todo!(),
            ArmInstruction::Blr { target } => {
                format!("blr {}", Into::<ArmRegister>::into(target))
            },
            ArmInstruction::Ldr { width, dest, src } => {
                match width {
                    ArmWidth::Word | ArmWidth::Double => format!("ldr {}, {}", dest, src),
                    _ => todo!()
                }
            },
            ArmInstruction::Mov { width, dest, src } => {
                format!("mov {}, {}", dest, src)
            },
            ArmInstruction::Ret => todo!(),
            ArmInstruction::Str { width, src, dest } => {
                match width {
                    ArmWidth::Word => format!("str {}, {}", src, dest),
                    ArmWidth::Double => format!("str {}, {}", src, dest),
                    _ => todo!("{:?}", width)
                }
            },
            ArmInstruction::Sub { dest, arg1, arg2 } => 
            {
                format!("sub {}, {}, {}", dest, arg1, arg2)
            },
            ArmInstruction::Sxtw { dest, src } => {
                format!("sxtw {}, {}", dest, src)
            },
            ArmInstruction::Bl { target } => {
                format!("bl {}", target)
            },
            ArmInstruction::Label { name } => {
                format!("{}:", name)
            },
            ArmInstruction::Directive { name, operands } => {
                format!(".{} {}", name, operands)
            }
        }
    }
}


impl Into<String> for ArmRegister {
    fn into(self) -> String {
        let s: &str = match (self.name, self.width) {
            (ArmRegisterName::Zero, ArmWidth::Word) => "wzr",
            (ArmRegisterName::Zero, ArmWidth::Double) => "xzr",
            (ArmRegisterName::Zero, _) => panic!("invalid width for zero register"),
            (ArmRegisterName::Pc, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::Pc, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::Pc, ArmWidth::Half) => todo!(),
            (ArmRegisterName::Pc, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::Pc, ArmWidth::Word) => todo!(),
            (ArmRegisterName::Pc, ArmWidth::Double) => todo!(),
            (ArmRegisterName::Sp, ArmWidth::Word) => "wsp",
            (ArmRegisterName::Sp, ArmWidth::Double) => "sp",
            (ArmRegisterName::Sp, _) => todo!(),
            (ArmRegisterName::Lr, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::Lr, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::Lr, ArmWidth::Half) => todo!(),
            (ArmRegisterName::Lr, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::Lr, ArmWidth::Word) => todo!(),
            (ArmRegisterName::Lr, ArmWidth::Double) => "lr",
            (ArmRegisterName::X0, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X0, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X0, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X0, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X0, ArmWidth::Word) => "w0",
            (ArmRegisterName::X0, ArmWidth::Double) => "x0",
            (ArmRegisterName::X1, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X1, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X1, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X1, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X1, ArmWidth::Word) => "w1",
            (ArmRegisterName::X1, ArmWidth::Double) => "x1",
            (ArmRegisterName::X2, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X2, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X2, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X2, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X2, ArmWidth::Word) => "w2",
            (ArmRegisterName::X2, ArmWidth::Double) => "x2",
            (ArmRegisterName::X3, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X3, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X3, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X3, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X3, ArmWidth::Word) => "w3",
            (ArmRegisterName::X3, ArmWidth::Double) => "x3",
            (ArmRegisterName::X4, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X4, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X4, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X4, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X4, ArmWidth::Word) => "w4",
            (ArmRegisterName::X4, ArmWidth::Double) => "x4",
            (ArmRegisterName::X5, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X5, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X5, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X5, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X5, ArmWidth::Word) => "w5",
            (ArmRegisterName::X5, ArmWidth::Double) => "x5",
            (ArmRegisterName::X6, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X6, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X6, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X6, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X6, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X6, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X7, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X7, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X7, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X7, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X7, ArmWidth::Word) => "w7",
            (ArmRegisterName::X7, ArmWidth::Double) => "x7",
            (ArmRegisterName::X8, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X8, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X8, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X8, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X8, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X8, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X9, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X9, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X9, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X9, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X9, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X9, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X10, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X10, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X10, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X10, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X10, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X10, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X11, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X11, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X11, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X11, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X11, ArmWidth::Word) => "w11",
            (ArmRegisterName::X11, ArmWidth::Double) => "x11",
            (ArmRegisterName::X12, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X12, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X12, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X12, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X12, ArmWidth::Word) => "w12",
            (ArmRegisterName::X12, ArmWidth::Double) => "x12",
            (ArmRegisterName::X13, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X13, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X13, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X13, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X13, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X13, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X14, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X14, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X14, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X14, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X14, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X14, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X15, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X15, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X15, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X15, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X15, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X15, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X16, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X16, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X16, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X16, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X16, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X16, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X17, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X17, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X17, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X17, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X17, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X17, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X18, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X18, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X18, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X18, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X18, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X18, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X19, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X19, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X19, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X19, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X19, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X19, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X20, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X20, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X20, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X20, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X20, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X20, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X21, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X21, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X21, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X21, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X21, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X21, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X22, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X22, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X22, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X22, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X22, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X22, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X23, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X23, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X23, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X23, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X23, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X23, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X24, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X24, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X24, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X24, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X24, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X24, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X25, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X25, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X25, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X25, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X25, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X25, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X26, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X26, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X26, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X26, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X26, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X26, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X27, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X27, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X27, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X27, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X27, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X27, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X28, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X28, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X28, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X28, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X28, ArmWidth::Word) => todo!(),
            (ArmRegisterName::X28, ArmWidth::Double) => todo!(),
            (ArmRegisterName::X29, ArmWidth::Byte) => todo!(),
            (ArmRegisterName::X29, ArmWidth::SignedByte) => todo!(),
            (ArmRegisterName::X29, ArmWidth::Half) => todo!(),
            (ArmRegisterName::X29, ArmWidth::SignedHalf) => todo!(),
            (ArmRegisterName::X29, ArmWidth::Word) => "w29",
            (ArmRegisterName::X29, ArmWidth::Double) => "x29",
        };
        s.to_string()
    }
}

impl Display for ArmRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x: String = self.clone().into();
        write!(f, "{}", x)
        // let s: String = self.into();
        // write!(f, "{}", s)
    }
}

impl Display for ArmVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmVal::Reg(arm_register) => arm_register.fmt(f),
            ArmVal::Imm(x) => write!(f, "{}", x),
            ArmVal::RegOffset(arm_register, offset) => {
                let double_reg = ArmRegister {
                    name: arm_register.name,
                    width: ArmWidth::Double
                };
                write!(f, "[{}, {}]", double_reg, offset)
            },
            ArmVal::LabelOffset(name, offset) => {
                match offset {
                    0 => write!(f, "{}", name),
                    9998 => write!(f, "{}", name), // %hi in riscv is adrp with no offset in arm
                    9999 => write!(f, ":lo12:{}", name), // reserved for 12 low bits of label addr
                    _ => write!(f, "[{}, {}]", name, offset)
                }
            }
        }
    }
}

impl Into<ArmRegister> for ArmRegisterName {
    fn into(self) -> ArmRegister {
        ArmRegister { width: ArmWidth::Double, name: self }
    }
}
