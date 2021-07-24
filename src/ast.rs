use std::convert::Infallible;
use std::str::FromStr;

pub enum NumberLiteral {
    U32(u32)
}


impl FromStr for NumberLiteral {
    // the lexer should guarantee this always works
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [number @ .., kind] = s.split('_').collect::<Vec<_>>().as_slice() {
            match *kind {
                "u32" => Ok(NumberLiteral::U32(u32::from_str(&number.join("")).unwrap())),
                _ => unimplemented!()
            }
        } else {
            unreachable!()
        }
    }
}