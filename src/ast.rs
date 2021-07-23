use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct FunctionDeclaration {
    name: Identifier,
    declared_type: Type,
    branches: Vec<Branch>,
}

impl FunctionDeclaration {
    pub fn new(name: Identifier, declared_type: Type, branches: Vec<Branch>) -> FunctionDeclaration {
        FunctionDeclaration {
            name,
            declared_type,
            branches,
        }
    }
}

#[derive(Debug)]
pub struct DataDeclaration {
    name: Identifier,
    // cannot repeat data
    data: HashSet<Expr>,
}

impl DataDeclaration {
    pub fn new(name: Identifier, data: Vec<Expr>) -> DataDeclaration {
        DataDeclaration {
            name,
            data: HashSet::from_iter(data),
        }
    }
}

#[derive(Debug)]
pub struct Branch {
    pattern: Vec<Expr>,
    action: Vec<Expr>,
}

impl Branch {
    pub fn new(pattern: Vec<Expr>, action: Vec<Expr>) -> Branch {
        Branch {
            pattern,
            action,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(str: String) -> Identifier {
        Identifier(str)
    }
}

#[derive(Debug)]
pub enum Type {
    Func(Box<Type>, Box<Type>),
    Iden(Identifier),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Expr {
    Iden(Identifier),
    Prim(Primitive),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Primitive {
    U32(u32)
}