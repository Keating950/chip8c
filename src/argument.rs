use crate::parser::Rule;
use pest::iterators::Pair;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone)]
pub struct Argument {
    val: Option<u16>,
    rule: Rule,
}

impl Argument {
    pub fn from_pair(p: Pair<Rule>) -> Result<Argument, ParseIntError> {
        let rule = p.as_rule();
        if matches!(rule, Rule::argument) {
            return Argument::from_pair(p.into_inner().nth(0).unwrap());
        }
        let val = match rule {
            Rule::immediate => {
                let inner: Pair<Rule> = p.into_inner().nth(0).unwrap();
                let (base, prefix) = match inner.as_rule() {
                    Rule::hex_number => (16, "0x"),
                    Rule::bin_number => (2, "0b"),
                    Rule::oct_number => (8, "0"),
                    Rule::dec_number => (10, ""),
                    _ => unreachable!(),
                };
                Some(u16::from_str_radix(
                    // FIXME: Some arguments can only be u8
                    inner.as_str().trim_start_matches(prefix),
                    base,
                )?)
            }
            Rule::register => Some(
                p.into_inner()
                    .nth(0)
                    .unwrap()
                    .as_str()
                    .trim_start_matches('v')
                    .parse::<u16>()?,
            ),
            _ => None,
        };
        Ok(Argument { val, rule })
    }

    pub fn as_rule(&self) -> Rule {
        self.rule
    }

    pub fn value(&self) -> Option<u16> {
        self.val
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::InstructionParser;
    use pest::Parser;

    fn run_test(r: Rule, arg: &str, check_rule: bool) {
        let pair = InstructionParser::parse(r, arg).unwrap().nth(0).unwrap();
        let arg = Argument::from_pair(pair);
        assert!(arg.is_ok());
        if check_rule {
            assert_eq!(arg.unwrap().as_rule(), r)
        }
    }

    #[test]
    fn test_immediate() {
        for arg in &["10", "0x10", "0b01010", "0724"] {
            run_test(Rule::immediate, arg, true);
        }
    }

    #[test]
    fn test_register() {
        for i in 0..0xF {
            let mut arg = String::from("v");
            arg.push_str(&i.to_string());
            run_test(Rule::register, &arg, true)
        }
    }

    #[test]
    fn test_special() {
        for arg in &["I", "[I]", "DT", "ST", "F", "B"] {
            run_test(Rule::argument, arg, false);
        }
    }
}
