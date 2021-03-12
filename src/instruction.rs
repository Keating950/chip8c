use crate::parser::{ParseInstructionError, Rule};
use lazy_static::lazy_static;
use pest::iterators::Pair;
use std::{collections::HashMap, iter::FromIterator, num::ParseIntError, ops::BitOr};

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
    pub fn from_pair(p: Pair<Rule>) -> Result<Instruction, ParseInstructionError> {
        let rule = p.as_rule();
        assert!(matches!(rule, Rule::inst));
        let mut inner: Vec<Pair<Rule>> = p.into_inner().collect();
        let code = Instruction::get_code(&inner);
        let args: Result<Vec<Option<u16>>, ParseIntError> = inner
            .drain(1..)
            .enumerate()
            .map(|(pos, arg)| Instruction::parse_arg(arg, pos))
            .collect();
        let arr = args?;
        debug_assert!(arr.len() < 4);
        Ok(Instruction { base: code, args: [arr[0], arr[1], arr[2]] })
    }

    fn parse_arg(p: Pair<Rule>, pos: usize) -> Result<Option<u16>, ParseIntError> {
        debug_assert!(matches!(p.as_rule(), Rule::argument));
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


    fn get_code(elems: &[Pair<Rule>]) -> u16 {
        debug_assert!(elems.len() >= 1);
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
        if let Some(c) = CODES.get(&(elems[0].as_str())) {
            return *c;
        }
        return match elems[0].as_str() {
            "ld" => Instruction::match_ld(&elems[1], &elems[2]),
            "add" => Instruction::match_ld(&elems[1], &elems[2]),
            _ => unreachable!(),
        };
    }

    fn match_ld(arg1: &Pair<Rule>, arg2: &Pair<Rule>) -> u16 {
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

    fn match_add(arg1: &Pair<Rule>, arg2: &Pair<Rule>) -> u16 {
        match (arg1.as_rule(), arg2.as_rule()) {
            (Rule::register, Rule::immediate) => 0x7000,
            (Rule::register, Rule::register) => 0x8004,
            (Rule::idx_register, Rule::register) => 0xF01E,
            _ => unreachable!(),
        }
    }


    pub fn as_opcode(&self) -> u16 {
        self.args
            .iter()
            .map(|arg| arg.unwrap_or(0))
            .fold(self.base, u16::bitor)
    }
}
