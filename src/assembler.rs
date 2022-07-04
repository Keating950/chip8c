use crate::{error::*, instruction::Instruction, parser::Rule};
use pest::iterators::Pair;
use std::{collections::HashMap, io::prelude::*};

#[derive(Debug)]
enum Item<'a> {
    Inst(Instruction),
    Label(&'a str),
}

#[derive(Debug)]
pub struct Assembler<'a> {
    instructions: Vec<Instruction>,
    labels: HashMap<&'a str, u16>,
}

impl<'a> Assembler<'a> {
    const PROGRAM_START: u16 = 0x200;

    pub fn build(pairs: impl Iterator<Item = Pair<'a, Rule>>) -> Result<Assembler<'a>> {
        let mut asm = Assembler {
            instructions: Default::default(),
            labels: Default::default(),
        };
        for p in pairs {
            if p.as_rule() == Rule::EOF {
                break;
            }
            match asm.parse_item(p)? {
                Item::Inst(inst) => asm.instructions.push(inst),
                Item::Label(lbl) => {
                    if asm
                        .labels
                        .insert(
                            lbl,
                            (asm.instructions.len() * 2) as u16 + Assembler::PROGRAM_START,
                        )
                        .is_some()
                    {
                        return Err(Error::DuplicateLabel(lbl.to_string()));
                    }
                }
            }
        }
        asm.resolve_args()?;
        Ok(asm)
    }

    pub fn write_bin(&self, mut dest: impl Write) -> Result<()> {
        for inst in &self.instructions {
            dest.write_all(&inst.as_bytes()?)?;
        }
        Ok(())
    }

    fn parse_item(&mut self, p: Pair<'a, Rule>) -> Result<Item<'a>> {
        match p.as_rule() {
            Rule::label | Rule::elem => self.parse_item(p.into_inner().next().unwrap()),
            Rule::instruction => Ok(Item::Inst(Instruction::try_from(p)?)),
            Rule::label_inner => Ok(Item::Label(p.as_str())),
            other => Err(Error::Internal(format!(
                "Assembler::parse_item recieved a Pair with Rule type {:?}",
                other
            ))),
        }
    }

    fn resolve_args(&mut self) -> Result<()> {
        for inst in self.instructions.iter_mut() {
            if let Some(lbl) = inst.unresolved_arg() {
                let addr = self
                    .labels
                    .get(lbl)
                    .ok_or_else(|| Error::UnresolvedLabel(lbl.to_string()))?;
                inst.resolve_arg(*addr)?;
            }
        }
        Ok(())
    }
}
