#![feature(iter_intersperse)]
#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub order);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_hello() {
        println!("hello");
    }
}