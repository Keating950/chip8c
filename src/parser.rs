use crate::error::*;
use pest::{iterators::Pair, Parser as ParserTrait};
use pest_derive::Parser as ParserDerive;

#[derive(ParserDerive)]
#[grammar = "grammar.pest"]
pub struct Parser;

impl Parser {
    pub fn parse(text: &str) -> Result<impl Iterator<Item = Pair<'_, Rule>>> {
        <Parser as ParserTrait<Rule>>::parse(Rule::prog, text).map_err(|e| e.into())
    }
}
