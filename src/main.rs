mod address;
mod args;
mod assembler;
mod error;
mod instruction;
mod parser;
mod register;
use crate::{args::Args, assembler::Assembler, error::*, parser::Parser};
use std::fs;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}

fn try_main() -> Result<()> {
    let args = Args::parse();
    let text = fs::read_to_string(&args.input)?;
    let asm = Assembler::build(Parser::parse(&text)?)?;
    let output = match &args.output {
        Some(p) => fs::OpenOptions::new().write(true).open(&p),
        None => {
            let mut out = args.input.clone();
            out.set_extension("bin");
            fs::OpenOptions::new().write(true).create(true).open(&out)
        }
    }?;
    asm.write_bin(&output)?;
    Ok(())
}

#[cfg(test)]
mod test_macros {
    #[macro_export]
    macro_rules! assert_ok {
        ($val:ident) => {{
            assert!($val.is_ok(), "{:?}", $val.unwrap_err());
            $val.unwrap()
        }};
        ($e:expr) => {{
            let val = $e;
            assert_ok!(val)
        }};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_ok;

    #[test]
    fn test_assemble() {
        let mut dest: Vec<u8> = Default::default();
        let progs = [
            include_str!("../test_files/instructions.asm"),
            include_str!("../test_files/labels.asm"),
        ];
        for text in progs {
            let parsed = match Parser::parse(text) {
                Ok(iter) => iter,
                Err(e) => {
                    assert!(false, "{:?}", e);
                    unreachable!()
                }
            };
            let asm = assert_ok!(Assembler::build(parsed));
            assert_ok!(asm.write_bin(&mut dest));
            dest.clear();
        }
    }
}
