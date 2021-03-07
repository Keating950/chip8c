use pest::{Parser, iterators::Pair};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct InstructionParser;

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
    fn test_unary() {
        run_instruction_test(&["cls", "ret"]);
    }

    #[test]
    fn test_reg_imm() {
        run_instruction_test(&["se V0 0x001", "sne V1 0x01"]);
    }

    #[test]
    fn test_comma() {
        run_instruction_test(&["se V0, 0x001", "sne, V1, 0x01"]);
    }

    #[test]
    fn test_bases() {
        run_instruction_test(&["se V0 0b1", "se V0 01", "se V0 1", "se V0 0x1"]);
    }

    #[test]
    fn test_comment() {
        let parse_result = InstructionParser::parse(Rule::line, "# this is a comment");
        assert!(parse_result.is_ok());
        let parse_result = InstructionParser::parse(Rule::inst, "se V0 01 # so is this");
        assert!(parse_result.is_ok())
    }

}