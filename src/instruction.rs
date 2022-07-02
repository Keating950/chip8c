use crate::{error::*, parser::Rule, register::Register};
use pest::iterators::Pair;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    AddI {
        reg: Register,
    },
    AddReg {
        dest: Register,
        src: Register,
    },
    AddImm {
        reg: Register,
        imm: u8,
    },
    And {
        dest: Register,
        src: Register,
    },
    Call {
        addr: u16,
    },
    Cls,
    Drw {
        x: Register,
        y: Register,
        nibble: u8,
    },
    JpRel {
        addr: u16,
    },
    JpAbs {
        addr: u16,
    },
    LdBcd {
        reg: Register,
    },
    LdSetDt {
        reg: Register,
    },
    LdSetSt {
        reg: Register,
    },
    LdSprite {
        reg: Register,
    },
    LdAddr {
        addr: u16,
    },
    LdReadDt {
        reg: Register,
    },
    LdKey {
        reg: Register,
    },
    LdReg {
        dest: Register,
        src: Register,
    },
    LdRegDump {
        reg: Register,
    },
    LdImm {
        reg: Register,
        imm: u8,
    },
    LdRegRead {
        reg: Register,
    },
    Or {
        dest: Register,
        src: Register,
    },
    Ret,
    Rnd {
        reg: Register,
        imm: u8,
    },
    SeReg {
        reg0: Register,
        reg1: Register,
    },
    SeImm {
        reg: Register,
        imm: u8,
    },
    Shl {
        reg: Register,
    },
    Shr {
        reg: Register,
    },
    Sknp {
        reg: Register,
    },
    Skp {
        reg: Register,
    },
    SneReg {
        reg0: Register,
        reg1: Register,
    },
    SneImm {
        reg: Register,
        imm: u8,
    },
    Sub {
        dest: Register,
        src: Register,
    },
    SubN {
        dest: Register,
        src: Register,
    },
    Sys {
        addr: u16,
    },
    Xor {
        dest: Register,
        src: Register,
    },
}

impl TryFrom<Pair<'_, Rule>> for Instruction {
    type Error = Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self> {
        use Instruction::*;
        use Rule::*;
        let rule = value.as_rule();
        let mut inner = value.into_inner();
        match rule {
            instruction => Instruction::try_from(inner.next().unwrap()),
            add_reg => Ok(AddReg {
                dest: inner.next().unwrap().try_into()?,
                src: inner.next().unwrap().try_into()?,
            }),
            add_idx => Ok(AddI {
                reg: inner.next().unwrap().try_into()?,
            }),
            add_imm => Ok(AddImm {
                reg: inner.next().unwrap().try_into()?,
                imm: parse_imm(inner.next().unwrap())?,
            }),
            and => Ok(And {
                dest: inner.next().unwrap().try_into()?,
                src: inner.next().unwrap().try_into()?,
            }),
            call => Ok(Call {
                addr: parse_imm(inner.next().unwrap())?,
            }),
            cls => Ok(Cls),
            drw => {
                let (x, y, nibble) = (
                    inner.next().unwrap().try_into()?,
                    inner.next().unwrap().try_into()?,
                    parse_imm(inner.next().unwrap())?,
                );
                if nibble > 0b1111 {
                    Err(Error::ExceedBounds(nibble as u16, 0b1111))
                } else {
                    Ok(Drw { x, y, nibble })
                }
            }
            jp_rel => Ok(JpRel {
                addr: parse_imm(inner.next().unwrap())?,
            }),
            jp_abs => Ok(JpAbs {
                addr: parse_imm(inner.next().unwrap())?,
            }),
            or => Ok(Or {
                dest: inner.next().unwrap().try_into()?,
                src: inner.next().unwrap().try_into()?,
            }),
            ret => Ok(Ret),
            rnd => Ok(Rnd {
                reg: inner.next().unwrap().try_into()?,
                imm: parse_imm(inner.next().unwrap())?,
            }),
            se => Ok(SeReg {
                reg0: inner.next().unwrap().try_into()?,
                reg1: inner.next().unwrap().try_into()?,
            }),
            se_imm => Ok(SeImm {
                reg: inner.next().unwrap().try_into()?,
                imm: parse_imm(inner.next().unwrap())?,
            }),
            shl => Ok(Shl {
                reg: inner.next().unwrap().try_into()?,
            }),
            shr => Ok(Shr {
                reg: inner.next().unwrap().try_into()?,
            }),
            skp => Ok(Skp {
                reg: inner.next().unwrap().try_into()?,
            }),
            sknp => Ok(Sknp {
                reg: inner.next().unwrap().try_into()?,
            }),
            sne => Ok(SneReg {
                reg0: inner.next().unwrap().try_into()?,
                reg1: inner.next().unwrap().try_into()?,
            }),
            sne_imm => Ok(SneImm {
                reg: inner.next().unwrap().try_into()?,
                imm: parse_imm(inner.next().unwrap())?,
            }),
            sub => Ok(Sub {
                dest: inner.next().unwrap().try_into()?,
                src: inner.next().unwrap().try_into()?,
            }),
            subn => Ok(SubN {
                dest: inner.next().unwrap().try_into()?,
                src: inner.next().unwrap().try_into()?,
            }),
            sys => Ok(Sys {
                addr: parse_imm(inner.next().unwrap())?,
            }),
            xor => Ok(Xor {
                dest: inner.next().unwrap().try_into()?,
                src: inner.next().unwrap().try_into()?,
            }),
            // LD
            ld_bcd => Ok(LdBcd {
                reg: inner.next().unwrap().try_into()?,
            }),
            ld_set_dt => Ok(LdSetDt {
                reg: inner.next().unwrap().try_into()?,
            }),
            ld_sprite => Ok(LdSprite {
                reg: inner.next().unwrap().try_into()?,
            }),
            ld_i_imm => Ok(LdAddr {
                addr: parse_imm(inner.next().unwrap())?,
            }),
            ld_set_st => Ok(LdSetSt {
                reg: inner.next().unwrap().try_into()?,
            }),
            ld_read_dt => Ok(LdReadDt {
                reg: inner.next().unwrap().try_into()?,
            }),
            ld_read_key => Ok(LdKey {
                reg: inner.next().unwrap().try_into()?,
            }),
            ld_reg => Ok(LdReg {
                dest: inner.next().unwrap().try_into()?,
                src: inner.next().unwrap().try_into()?,
            }),
            ld_i_reg => Ok(LdImm {
                reg: inner.next().unwrap().try_into()?,
                imm: parse_imm(inner.next().unwrap())?,
            }),
            ld_reg_dump => Ok(LdRegDump {
                reg: inner.next().unwrap().try_into()?,
            }),
            ld_reg_read => Ok(LdRegRead {
                reg: inner.next().unwrap().try_into()?,
            }),
            other => Err(Error::Internal(format!(
                "Cannot parse a Pair with Rule type {:?} as an Instruction",
                other
            ))),
        }
    }
}

impl Instruction {
    pub fn to_bytes(self) -> [u8; 2] {
        use Instruction::*;
        match self {
            Sys { addr } => addr.to_be_bytes(),
            Cls => [0, 0xE0],
            Ret => [0, 0xEE],
            JpAbs { addr } => (0x1000 | addr).to_be_bytes(),
            Call { addr } => (0x2000 | addr).to_be_bytes(),
            SeImm { reg, imm } => [0x30 | reg as u8, imm],
            SneImm { reg, imm } => [0x40 | reg as u8, imm],
            SeReg { reg0, reg1 } => [0x50 | reg0 as u8, (reg1 as u8) << 4],
            LdImm { reg, imm } => [0x60 | reg as u8, imm],
            AddImm { reg, imm } => [0x70 | reg as u8, imm],
            LdReg { dest, src } => [0x80 | dest as u8, (src as u8) << 4],
            Or { dest, src } => [0x80 | dest as u8, ((src as u8) << 4) + 1],
            And { dest, src } => [0x80 | dest as u8, ((src as u8) << 4) + 2],
            Xor { dest, src } => [0x80 | dest as u8, ((src as u8) << 4) + 3],
            AddReg { dest, src } => [0x80 | dest as u8, ((src as u8) << 4) + 4],
            Sub { dest, src } => [0x80 | dest as u8, ((src as u8) << 4) + 5],
            Shr { reg } => [0x80 | reg as u8, 0x06],
            SubN { dest, src } => [0x80 | dest as u8, ((src as u8) << 4) + 7],
            Shl { reg } => [0x80 | reg as u8, 0x0E],
            SneReg { reg0, reg1 } => [0x90 | reg0 as u8, (reg1 as u8) << 4],
            LdAddr { addr } => (0xA000 | addr).to_be_bytes(),
            JpRel { addr } => (0xB000 | addr).to_be_bytes(),
            Rnd { reg, imm } => [0xC0 | reg as u8, imm],
            Drw { x, y, nibble } => [0xD0 | x as u8, ((y as u8) << 4) | nibble],
            Skp { reg } => [0xE0 | reg as u8, 0x9E],
            Sknp { reg } => [0xE0 | reg as u8, 0xA1],
            LdReadDt { reg } => [0xF0 | reg as u8, 0x07],
            LdKey { reg } => [0xF0 | reg as u8, 0x0A],
            LdSetDt { reg } => [0xF0 | reg as u8, 0x15],
            LdSetSt { reg } => [0xF0 | reg as u8, 0x18],
            AddI { reg } => [0xF0 | reg as u8, 0x1E],
            LdSprite { reg } => [0xF0 | reg as u8, 0x29],
            LdBcd { reg } => [0xF0 | reg as u8, 0x33],
            LdRegDump { reg } => [0xF0 | reg as u8, 0x55],
            LdRegRead { reg } => [0xF0 | reg as u8, 0x65],
        }
    }
}

trait ParseImm: Sized + Into<u16> + Copy {
    fn from_str_radix(src: &str, radix: u32) -> std::result::Result<Self, ParseIntError>;
}
impl ParseImm for u8 {
    fn from_str_radix(src: &str, radix: u32) -> std::result::Result<Self, ParseIntError> {
        Self::from_str_radix(src, radix)
    }
}
impl ParseImm for u16 {
    fn from_str_radix(src: &str, radix: u32) -> std::result::Result<Self, ParseIntError> {
        Self::from_str_radix(src, radix)
    }
}

fn parse_imm<T: ParseImm>(p: Pair<'_, Rule>) -> Result<T> {
    let (base, prefix) = match p.as_rule() {
        Rule::imm => return parse_imm(p.into_inner().next().unwrap()),
        Rule::hex_lit => (16, "0x"),
        Rule::decimal_lit => (10, ""),
        Rule::octal_lit => (8, "0"),
        Rule::bin_lit => (2, "0b"),
        other => {
            return Err(Error::Internal(format!(
                "Passed a pair with rule type {:?} to parse_imm",
                other,
            )));
        }
    };
    let val =
        T::from_str_radix(p.as_str().trim_start_matches(prefix), base).map_err(Error::NumParse)?;
    if val.into() > 0x0FFF {
        Err(Error::ExceedBounds(val.into(), 0x0FFF))
    } else {
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() {
        use Instruction::*;
        let instructions = [
            Or {
                dest: Register::VA,
                src: Register::VB,
            },
            And {
                dest: Register::VA,
                src: Register::VB,
            },
        ];
        for inst in instructions {
            let val = u16::from_be_bytes(inst.to_bytes());
            println!("{:04X}", val);
        }
    }
}
