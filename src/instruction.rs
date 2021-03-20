use crate::{
    argument::Argument,
    parser::{ParseInstructionError, Rule},
};
use lazy_static::lazy_static;
use pest::iterators::Pair;
use std::{
    collections::HashMap,
    iter::FromIterator,
};

#[derive(Debug)]
pub struct Instruction {
    base: u16,
    args: [Option<Argument>; 3],
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
        let mut args: Vec<Option<Argument>> = parts.drain(1..).try_fold(
            Vec::with_capacity(3),
            |mut acc, elem| -> Result<Vec<Option<Argument>>, ParseInstructionError> {
                let arg = Argument::from_pair(elem)?;
                acc.push(Some(arg));
                Ok(acc)
            },
        )?;
        args.resize_with(3, || None);
        Ok(Instruction {
            base: code,
            args: [args[0], args[1], args[2]],
        })
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
                ("or", 0x8001),
                ("shr", 0x8006),
                ("shl", 0x800E),
                ("se", 0x3000),
                ("jp", 0xB000),
                ("rnd", 0xC000),
                ("drw", 0xD000),
            ]);
        }
        match CODES.get(mnemonic) {
            Some(c) => *c,
            None => Instruction::match_varying_args(mnemonic, &args[0], &args[1]),
        }
    }

    fn match_varying_args(mnemonic: &str, arg1: &Pair<Rule>, arg2: &Pair<Rule>) -> u16 {
        let arg_rules = (arg1.as_rule(), arg2.as_rule());
        match mnemonic {
            "ld" => match arg_rules {
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
            },
            "add" => match arg_rules {
                (Rule::register, Rule::immediate) => 0x7000,
                (Rule::register, Rule::register) => 0x8004,
                (Rule::idx_register, Rule::register) => 0xF01E,
                _ => unreachable!(),
            },
            "se" => match arg_rules {
                (Rule::register, Rule::immediate) => 0x3000,
                (Rule::register, Rule::register) => 0x5000,
                _ => unreachable!(),
            },
            "sne" => match arg_rules {
                (Rule::register, Rule::immediate) => 0x4000,
                (Rule::register, Rule::register) => 0x9000,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn as_opcode(&self) -> u16 {
        let mut base = self.base;
        let shift = |pos, n| n << (4 * (3 - (pos + 1)));
        for (i, arg_opt) in self.args.iter().enumerate() {
            let arg = match arg_opt {
                Some(val) => val,
                None => break,
            };
            match arg.as_rule() {
                Rule::register => base |= shift(i, arg.value().unwrap()),
                Rule::immediate => base |= arg.value().unwrap(),
                _ => ()
            }
        }
        base
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
