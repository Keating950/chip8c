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

#[macro_export]
macro_rules! static_assert {
    ($cond:expr) => {{
        #[allow(unused)]
        const fn static_assertion() {
            assert!($cond);
        }
        const _: () = static_assertion();
    }};
}
