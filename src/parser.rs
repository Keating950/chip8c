use crate::instruction::Instruction;
use pest::{
    iterators::Pairs,
    Parser,
};
use pest_derive::*;
use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub enum ParseInstructionError {
    BadInstruction(pest::error::Error<Rule>),
    BadArg(ParseIntError),
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseInstructionError::BadInstruction(s) =>
                write!(f, "Failed to parse {} as an instruction", s),
            ParseInstructionError::BadArg(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ParseInstructionError {}

impl From<ParseIntError> for ParseInstructionError {
    fn from(e: ParseIntError) -> Self {
        ParseInstructionError::BadArg(e)
    }
}

impl From<pest::error::Error<Rule>> for ParseInstructionError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        ParseInstructionError::BadInstruction(e)
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct InstructionParser;

impl InstructionParser {
    pub fn parse_buffer(s: &str) -> Result<Vec<Instruction>, ParseInstructionError> {
        let mut out = Vec::with_capacity(s.len() / 8); // rough estimate of chars per line
        for ln in s.lines() {
            let mut parsed: Pairs<'_, Rule> = InstructionParser::parse(Rule::line, &ln)?;
            match parsed.nth(0) {
                Some(inner) => match inner.as_rule() {
                    Rule::inst => out.push(Instruction::from_inst_pair(inner)?),
                    _ => {}
                },
                None => {}
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    fn run_test(instructions: &[&str]) {
        for e in instructions {
            let parse_result = InstructionParser::parse(Rule::inst, e);
            assert!(parse_result.is_ok())
        }
    }

    #[test]
    fn test_nullary() {
        run_test(&["cls", "ret"]);
    }

    #[test]
    fn test_unary() {
        run_test(&["jp 0x1000", "ret"]);
    }

    #[test]
    fn test_reg_imm() {
        run_test(&["se v0 0x001", "sne v1 0x01"]);
    }

    #[test]
    fn test_comma() {
        run_test(&["se v0, 0x001", "sne, v1, 0x01"]);
    }

    #[test]
    fn test_bases() {
        run_test(&["se v0 0b1", "se v0 01", "se v0 1", "se v0 0x1"]);
    }

    #[test]
    fn test_ld() {
        let ld_inst = [
            "ld v0 0x100",
            "ld v0 v1",
            "ld i, 0x1000",
            "ld v0, dt",
            "ld v3, k",
            "ld dt, v0",
            "ld st, v0",
            "ld f, v0",
            "ld b, v0",
            "ld [i], v0",
            "ld v0, [i]",
        ];
        run_test(&ld_inst);
    }

    #[test]
    fn test_add() {
        run_test(&["add v0 v1", "add v0 0x1000", "add i v0"])
    }

    #[test]
    fn test_comment() {
        let parse_result = InstructionParser::parse(Rule::line, "# this is a comment");
        assert!(parse_result.is_ok());
        let parse_result = InstructionParser::parse(Rule::inst, "se v0 01 # so is this");
        assert!(parse_result.is_ok())
    }
}
