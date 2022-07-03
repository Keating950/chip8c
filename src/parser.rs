use std::num::ParseIntError;

use crate::error::*;
use pest::{iterators::Pair, Parser as ParserTrait};
use pest_derive::Parser as ParserDerive;

#[derive(ParserDerive)]
#[grammar = "grammar.pest"]
pub struct Parser;

impl Parser {
    pub fn parse(text: &str) -> Result<impl Iterator<Item = Pair<'_, Rule>>> {
        Ok(<Parser as ParserTrait<Rule>>::parse(Rule::prog, text)?
            .next()
            .unwrap()
            .into_inner())
    }
}

pub trait ParseImm: Sized + Into<u16> + Copy {
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

pub fn parse_imm<T: ParseImm>(p: Pair<'_, Rule>) -> Result<T> {
    let (base, prefix) = match p.as_rule() {
        Rule::imm => return parse_imm(p.into_inner().next().unwrap()),
        Rule::hex_lit => (16, "0x"),
        Rule::dec_lit => (10, ""),
        Rule::oct_lit => (8, "0"),
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
