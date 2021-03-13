use crate::parser::{ParseInstructionError, Rule};
use lazy_static::lazy_static;
use pest::iterators::Pair;
use std::{
    collections::{HashMap, VecDeque},
    iter::FromIterator,
    num::ParseIntError,
    ops::BitOr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    base: u16,
    args: [Option<u16>; 3],
}

impl Instruction {
    fn recurse_rule(p: Pair<Rule>) -> Vec<Pair<Rule>> {
        match p.as_rule() {
            Rule::line | Rule::inst => Instruction::recurse_rule(p.into_inner().nth(0).unwrap()),
            Rule::nullary_inst => vec![p],
            Rule::binary_inst
            | Rule::variadic_inst
            | Rule::draw_inst
            | Rule::add_inst
            | Rule::ld_inst => p.into_inner().collect(),
            _ => unreachable!(),
        }
    }

    pub fn from_inst_pair(p: Pair<Rule>) -> Result<Instruction, ParseInstructionError> {
        let mut parts = Instruction::recurse_rule(p);
        let mnemonic = parts[0].as_str();
        let code = if parts.len() > 1 {
            Instruction::get_code(mnemonic, &parts[1..])
        } else {
            Instruction::get_code(mnemonic, &[])
        };
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
            Rule::register => Ok(Some(
                p.into_inner()
                    .nth(0)
                    .unwrap()
                    .as_str()
                    .trim_start_matches('v')
                    .parse::<u16>()?,
            )),

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

    fn or_register(base: u16, pos: u32, reg: u16) -> u16 {
        debug_assert!(pos < 3);
        base | (reg << (8 - 4 * pos))
    }

    pub fn as_opcode(&self) -> u16 {
        self.args
            .iter()
            .map(|arg| arg.unwrap_or(0))
            .enumerate()
            .fold(self.base, |base, (pos, arg)| {
                Instruction::or_register(base, pos as u32, arg)
            })
    }

    pub fn as_bytes(&self) -> (u8, u8) {
        let code = self.as_opcode();
        (((code & 0xFF00) >> 8) as u8, (code & 0xFF) as u8)
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

    macro_rules! hex_assert_eq {
        ($left:expr, $right:expr) => {
            assert_eq!($left, $right, "\nleft: {:#x} \nright: {:#x}", $left, $right)
        };
        ($left:expr, $right:expr, $fmt:literal, $($args:expr)*) => {
            assert_eq!($left, $right, concat!("\nleft: {:#x}\nright: {:#x}\n", $fmt), $left, $right, $($args)*)
        }

    }

    fn test_opcodes(insts: &[(&str, u16)]) {
        for (i, code) in insts {
            let pair: Pair<Rule> = get_inst(i);
            let inst = Instruction::from_inst_pair(pair).unwrap();
            let bytes = inst.as_bytes();
            let opcode = ((bytes.0 as u16) << 8) | (bytes.1 as u16);
            if opcode != *code {
                eprintln!()
            }
            hex_assert_eq!(opcode, *code);
            hex_assert_eq!(opcode, inst.as_opcode(), "INSTRUCTION: {}", i);
        }
    }

    #[test]
    fn test_nullary() {
        test_opcodes(&[("cls", 0x00E0), ("ret", 0x00EE)]);
    }

    #[test]
    fn test_binary() {
        test_opcodes(&[
            ("se v7 0xFF", 0x37FF),
            ("sne v7 0xFF", 0x47FF),
            ("or v4 v5", 0x8451),
        ]);
    }

    #[test]
    fn test_draw() {
        test_opcodes(&[("drw v1 v2 0xf", 0xD12F)]);
    }
}
