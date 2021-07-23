#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(destructuring_assignment)]

mod ast;

#[macro_use]
extern crate lalrpop_util;

fn main() {
    println!("hello world")
}

lalrpop_mod!(pub order);


#[cfg(test)]
mod tests {
    use crate::order::*;
    use crate::order::FunctionDeclarationParser;

    #[test]
    fn parse_fn() {
        let parsed = FunctionDeclarationParser::new().parse("add :: Int -> Int | a, b = plus, a, b").unwrap();
        println!("{:#?}", parsed);
    }

    #[test]
    fn parse_fn_with_primitives() {
        let parsed = FunctionDeclarationParser::new().parse("addTwo :: Int -> Int | a = plus, a, 2_u32").unwrap();
        println!("{:#?}", parsed);
    }

    #[test]
    fn parse_data_declaration() {
        let parsed = DataDeclarationParser::new().parse("data Bool = 0_u32 | 1_u32").unwrap();
        println!("{:#?}", parsed);
    }
}
