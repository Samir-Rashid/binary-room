use std::str::FromStr;
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

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addi" => Ok(Instruction::Addi),
            "sd" => Ok(Instruction::Sd),
            "ld" => Ok(Instruction::Ld),
            "sw" => Ok(Instruction::Sw),
            "lw" => Ok(Instruction::Lw),
            "mv" => Ok(Instruction::Mv),
            "addw" => Ok(Instruction::Addw),
            "sext.w" => Ok(Instruction::SextW),
            "jr" => Ok(Instruction::Jr),
            "li" => Ok(Instruction::Li),
            "add" => Ok(Instruction::Add),
            "sub" => Ok(Instruction::Sub),
            "mov" => Ok(Instruction::Mov),
            "ldr" => Ok(Instruction::Ldr),
            "str" => Ok(Instruction::Str),
            "b" => Ok(Instruction::B),
            "bl" => Ok(Instruction::Bl),
            "bx" => Ok(Instruction::Bx),
            "cmp" => Ok(Instruction::Cmp),
            "beq" => Ok(Instruction::Beq),
            _ => Err(()),
        }
    }
}
