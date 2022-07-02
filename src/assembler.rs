use crate::{error::*, instruction::Instruction, parser::Rule};
use pest::iterators::Pair;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Assembler<'a> {
    instructions: Vec<Instruction>,
    labels: HashMap<&'a str, usize>,
}

impl<'a> Assembler<'a> {
    pub fn build(pairs: impl Iterator<Item = Pair<'a, Rule>>) -> Result<Assembler<'a>> {
        let mut asm = Assembler {
            instructions: Default::default(),
            labels: Default::default(),
        };
        for p in pairs {
            asm.parse_item(p)?;
        }
        Ok(asm)
    }

    fn parse_item(&mut self, p: Pair<'a, Rule>) -> Result<()> {
        match p.as_rule() {
            Rule::label | Rule::elem => self.parse_item(p.into_inner().next().unwrap()),
            Rule::instruction => {
                self.instructions.push(Instruction::try_from(p)?);
                Ok(())
            }
            Rule::label_inner => {
                let lbl = p.as_str();
                if self.labels.contains_key(lbl) {
                    Err(Error::DuplicateLabel(lbl.to_owned()))
                } else {
                    self.labels.insert(lbl, self.instructions.len() * 2);
                    Ok(())
                }
            }
            other => Err(Error::Internal(format!(
                "Assembler::parse_item recieved a Pair with Rule type {:?}",
                other
            ))),
        }
    }
}
