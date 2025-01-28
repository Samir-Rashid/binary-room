/// This file defines all the supported ARM and RISC-V instructions we support.
/// We use `strum` to assist in serializing asm files to our [`Instruction`] enum.
///
/// We do not aim for completness of translating every possible instruction,
/// but we do want to thoroughly test for correctness.
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Instruction {
    // RISC-V Instructions
    #[strum(serialize = "addi")]
    Addi,
    #[strum(serialize = "sd")]
    Sd,
    #[strum(serialize = "ld")]
    Ld,
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

    // ARM Instructions
    #[strum(serialize = "add")]
    Add,
    #[strum(serialize = "sub")]
    Sub,
    #[strum(serialize = "mov")]
    Mov,
    #[strum(serialize = "ldr")]
    Ldr,
    #[strum(serialize = "str")]
    Str,
    #[strum(serialize = "b")]
    B,
    #[strum(serialize = "bl")]
    Bl,
    #[strum(serialize = "bx")]
    Bx,
    #[strum(serialize = "cmp")]
    Cmp,
    #[strum(serialize = "beq")]
    Beq,
}

