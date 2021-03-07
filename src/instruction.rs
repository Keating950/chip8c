use bobbin_bits::{U4, U12};
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt, iter::FromIterator, num::ParseIntError};

#[derive(Debug)]
pub enum ParseInstructionError {
    BadInstruction(String),
    WrongNumberArgs,
    BadArg(ParseIntError),
}

macro_rules! parse_err {
    ($kind:tt) => {
        Err(ParseInstructionError::$kind)
    };
    ($kind:tt, $val:expr) => {
        Err(ParseInstructionError::$kind($val))
    };
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseInstructionError::BadInstruction(s) =>
                write!(f, "Failed to parse {} as an instruction", s),
            ParseInstructionError::WrongNumberArgs =>
                write!(f, "Wrong number of aarguments provided to instruction"),
            ParseInstructionError::BadArg(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for ParseInstructionError {}

impl From<ParseIntError> for ParseInstructionError {
    fn from(e: ParseIntError) -> Self { ParseInstructionError::BadArg(e) }
}

type ParseResult = Result<u16, ParseInstructionError>;

pub fn parse_instruction(inst: &str) -> ParseResult {
    #[rustfmt::skip]

    let tokens: Vec<&str> = inst.trim().split_ascii_whitespace().collect();
    if !(1..=3).contains(&tokens.len()) {
        return parse_err!(BadInstruction, inst.to_string());
    }

    match tokens[0] {
        "CLS" => Ok(0x00E0),
        "RET" => Ok(0x00EE),
        _ => todo!(),
    }
}

/* lazy_static! {
    static ref INSTRUCTION_PARSERS: HashMap<&'static str, fn(U4, &[&str]) -> ParseResult> =
        HashMap::from_iter(vec![
            ("SYS", )

        ])
} */

fn parse_unary_instruction(nibble: U4, args: &[&str]) -> ParseResult {
    let mut opcode: u16 = (nibble as u16) << 12u16;
    if args.len() != 1 {
        return parse_err!(WrongNumberArgs);
    }
    Ok(opcode & parse_int_literal(args[0])?)
}

fn parse_int_literal(s: &str) -> Result<u16, ParseIntError> {
    const PREFIXES: [(&'static str, u32); 3] = [("0x", 16), ("0b", 2), ("0", 8)];
    for (pref, base) in &PREFIXES {
        if let Some(subs) = s.strip_prefix(pref) {
            return u16::from_str_radix(subs, *base);
        }
    }
    s.parse()
}
