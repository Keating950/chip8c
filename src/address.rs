use crate::{
    error::*,
    parser::{parse_imm, Rule},
};
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Address {
    Label(String),
    Short(u16),
}

impl TryFrom<Pair<'_, Rule>> for Address {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self> {
        match value.as_rule() {
            Rule::addr => Address::try_from(value.into_inner().next().unwrap()),
            Rule::imm | Rule::hex_lit | Rule::dec_lit | Rule::oct_lit | Rule::bin_lit => {
                Ok(Address::Short(parse_imm(value)?))
            }
            other => Err(Error::Internal(format!(
                "Cannot parse an Address from a Pair with Rule {:?}",
                other
            ))),
        }
    }
}

impl Address {
    pub fn label_value(&self) -> Option<&str> {
        match self {
            Address::Label(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn to_resolved(&self) -> Result<u16> {
        match self {
            Address::Label(s) => Err(Error::UnresolvedLabel(s.clone())),
            Address::Short(n) => Ok(*n),
        }
    }
}
