use crate::instruction::Instruction;
use pest::{iterators::Pair, Parser};
use pest_derive::*;
use std::{fmt, hint, num::ParseIntError};

#[derive(Debug)]
pub enum ParseInstructionError {
    BadInstruction(String),
    WrongNumberArgs,
    BadArg(ParseIntError),
}

type ParseResult = Result<u16, ParseInstructionError>;

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

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct InstructionParser;

impl InstructionParser {
    pub fn parse_file(s:&str) -> Result<Vec<Instruction>, ParseInstructionError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    fn run_instruction_test(instructions: &[&str]) {
        for e in instructions {
            let parse_result = InstructionParser::parse(Rule::inst, e);
            assert!(parse_result.is_ok())
        }
    }

    #[test]
    fn test_nullary() { run_instruction_test(&["cls", "ret"]); }

    #[test]
    fn test_unary() { 
        // run_instruction_test(&["cls", "ret"]); 
        let parse_result = InstructionParser::parse(Rule::inst, "jp 0x1000");
        dbg!(parse_result);
    }

    #[test]
    fn test_reg_imm() { run_instruction_test(&["se V0 0x001", "sne V1 0x01"]); }

    #[test]
    fn test_comma() { run_instruction_test(&["se V0, 0x001", "sne, V1, 0x01"]); }

    #[test]
    fn test_bases() { run_instruction_test(&["se V0 0b1", "se V0 01", "se V0 1", "se V0 0x1"]); }

    #[test]
    fn test_comment() {
        let parse_result = InstructionParser::parse(Rule::line, "# this is a comment");
        assert!(parse_result.is_ok());
        let parse_result = InstructionParser::parse(Rule::inst, "se V0 01 # so is this");
        assert!(parse_result.is_ok())
    }
}
