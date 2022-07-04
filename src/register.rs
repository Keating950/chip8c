use crate::{error::*, parser::Rule};
use pest::iterators::Pair;
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    V0 = 0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl TryFrom<u8> for Register {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self> {
        use Register::*;
        const REGISTERS: [Register; 16] = [
            V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF,
        ];
        if value <= 0xF {
            Ok(REGISTERS[value as usize])
        } else {
            Err(Error::Internal(
                "Could not convert u8 with value {value} to a Register".into(),
            ))
        }
    }
}
impl TryFrom<Pair<'_, Rule>> for Register {
    type Error = Error;
    fn try_from(value: Pair<Rule>) -> Result<Self> {
        match value.as_rule() {
            Rule::register => Register::try_from(value.into_inner().next().unwrap()),
            Rule::register_number => {
                let val = u8::from_str_radix(value.as_str(), 16).unwrap();
                Ok(Register::try_from(val).unwrap())
            }
            rule => Err(Error::Internal(format!(
                "Cannot parse a Register from a pair with rule type {:?}",
                rule
            ))),
        }
    }
}
