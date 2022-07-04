#![allow(dead_code)]
mod address;
mod args;
mod assembler;
mod error;
mod instruction;
mod parser;
mod register;

fn main() {
    println!("Hello, world!");
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
    use crate::{assembler::Assembler, assert_ok, parser::Parser};

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
            assert_ok!(asm.write_bin::<&mut Vec<u8>>(&mut dest));
            dest.clear();
        }
    }
}
