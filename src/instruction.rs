use crate::parser::{ParseInstructionError, Rule};
use lazy_static::lazy_static;
use pest::iterators::Pair;
use std::{
    collections::{HashMap, VecDeque},
    iter::FromIterator,
    num::ParseIntError,
    ops::BitOr,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Instruction {
    base: u16,
    args: [Option<u16>; 3],
}

#[macro_export]
macro_rules! bin_instruction {
    ($base:expr) => {
        BinaryInstruction { base: $base, args: [None; 3] }
    };
    ($base:expr, $arg0:expr) => {
        BinaryInstruction { base: $base, args: [Some($arg0), None, None] }
    };
    ($base:expr, $arg0:expr, $arg1:expr) => {
        BinaryInstruction {
            base: $base,
            args: [Some($arg0), Some($arg1), None],
        }
    };
    ($base:expr, $arg0:expr, $arg1:expr, $arg2:expr) => {
        BinaryInstruction {
            base: $base,
            args: [Some($arg0), Some($arg1), Some($arg2)],
        }
    };
}

impl Instruction {
    pub fn from_inst_pair(p: Pair<Rule>) -> Result<Instruction, ParseInstructionError> {
        assert!(matches!(p.as_rule(), Rule::inst));
        let mut parts: Vec<Pair<Rule>> = p.into_inner().nth(0).unwrap().into_inner().collect();
        let mnemonic = parts[0].as_str();
        let code = Instruction::get_code(mnemonic, &parts[1..]);
        let args: Result<Vec<Option<u16>>, _> = parts
            .drain(1..)
            .enumerate()
            .map(|(pos, arg)| Instruction::parse_arg(arg, pos))
            .collect();
        let mut args_ok = args?;
        args_ok.resize_with(3, || None);
        Ok(Instruction {
            base: code,
            args: [args_ok[0], args_ok[1], args_ok[2]],
        })
    }

    fn parse_arg(p: Pair<Rule>, pos: usize) -> Result<Option<u16>, ParseIntError> {
        match p.as_rule() {
            Rule::immediate => {
                let inner: Pair<Rule> = p.into_inner().nth(0).unwrap();
                let (base, prefix) = match inner.as_rule() {
                    Rule::hex_number => (16, "0x"),
                    Rule::bin_number => (2, "0b"),
                    Rule::oct_number => (8, "0"),
                    Rule::dec_number => (10, ""),
                    _ => unreachable!(),
                };
                let foo = u16::from_str_radix(inner.as_str().trim_start_matches(prefix), base)?;
                Ok(Some(foo))
            }
            Rule::register => {
                let shift_amount = 12 - (4 * pos);
                let arg = p
                    .into_inner()
                    .nth(0)
                    .unwrap()
                    .as_str()
                    .trim_start_matches('v')
                    .parse::<u16>()?;
                Ok(Some(arg << shift_amount))
            }

            _ => Ok(None), // special args are included in the base opcode
        }
    }


    fn get_code(mnemonic: &str, args: &[Pair<Rule>]) -> u16 {
        lazy_static! {
            static ref CODES: HashMap<&'static str, u16> = HashMap::from_iter(vec![
                ("cls", 0x00E0),
                ("ret", 0x00EE),
                ("sys", 0x0000),
                ("jp", 0x1000),
                ("call", 0x2000),
                ("skp", 0xE09E),
                ("sknp", 0xE0A1),
                ("shr", 0x8006),
                ("shl", 0x800E),
                ("se", 0x3000),
                ("sne", 0x5000),
                ("jp", 0xB000),
                ("rnd", 0xC000),
                ("drw", 0xD000),
            ]);
        }
        if let Some(c) = CODES.get(mnemonic) {
            return *c;
        }
        return match mnemonic {
            "ld" => Instruction::match_ld_args(&args[0], &args[1]),
            "add" => Instruction::match_add_args(&args[0], &args[1]),
            _ => unreachable!(),
        };
    }

    fn match_ld_args(arg1: &Pair<Rule>, arg2: &Pair<Rule>) -> u16 {
        let (r1, r2) = (arg1.as_rule(), arg2.as_rule());
        debug_assert!(matches!(r1, Rule::argument) && matches!(r2, Rule::argument));
        match (arg1.as_rule(), arg2.as_rule()) {
            (Rule::register, Rule::immediate) => 0x6000,
            (Rule::register, Rule::register) => 0x8000,
            (Rule::idx_register, Rule::immediate) => 0xA000,
            (Rule::register, Rule::delay_timer) => 0xF007,
            (Rule::register, Rule::keyboard) => 0xF00A,
            (Rule::delay_timer, Rule::register) => 0xF015,
            (Rule::sound_timer, Rule::register) => 0xF018,
            (Rule::font, Rule::register) => 0xF029,
            (Rule::bcd, Rule::register) => 0xF033,
            (Rule::idx_register, Rule::register) => 0xF055,
            (Rule::register, Rule::idx_register) => 0xF065,
            _ => unreachable!(),
        }
    }

    fn match_add_args(arg1: &Pair<Rule>, arg2: &Pair<Rule>) -> u16 {
        match (arg1.as_rule(), arg2.as_rule()) {
            (Rule::register, Rule::immediate) => 0x7000,
            (Rule::register, Rule::register) => 0x8004,
            (Rule::idx_register, Rule::register) => 0xF01E,
            _ => unreachable!(),
        }
    }

    // FIXME
    fn or_register(base: u16, pos: u32, reg: u16) -> u16 {
        base | (reg << (12 - 4 * pos))
    }

    // FIXME
    pub fn as_bytes(&self) -> (u8, u8) {
        let code = self
            .args
            .iter()
            .map(|arg| arg.unwrap_or(0))
            .enumerate()
            .fold(self.base, |base, (pos, arg)| {
                Instruction::or_register(base, pos as u32, arg)
            });
        let b1 = (code & 0b1111111100000000) as u8;
        let b2 = (code & 0x0F) as u8;
        (b1, b2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{instruction::Rule, InstructionParser};
    use pest::Parser;

    fn get_inst(ln: &str) -> Pair<Rule> {
        InstructionParser::parse(Rule::inst, ln)
            .unwrap()
            .nth(0)
            .unwrap()
    }

    fn run_test(lines: &[&str]) {
        for ln in lines {
            let pair: Pair<Rule> = get_inst(ln);
            assert!(Instruction::from_inst_pair(pair).is_ok());
        }
    }

    #[test]
    fn test_nullary() {
        let pair: Pair<Rule> = get_inst("cls");
        let inst = Instruction::from_inst_pair(pair).unwrap();
        let bytes = inst.as_bytes();
        let opcode = ((bytes.0 as u16) << 8) & (bytes.1 as u16);
        todo!();
        // assert_eq!(opcode, 0x00E0);
    }

    #[test]
    fn test_bin_reg_immediate() {
        let pair: Pair<Rule> = get_inst("se v0 0x100");
        let inst = Instruction::from_inst_pair(pair).unwrap();
        let bytes = inst.as_bytes();
        let opcode = ((bytes.0 as u16) << 8) & (bytes.1 as u16);
    }
}
