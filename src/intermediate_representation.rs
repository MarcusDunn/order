use dependant_sum_type::DependantSumType;
use dependent_product_type::DependentProductType;

use crate::intermediate_representation::binary_sum_type::BinarySumType;

mod binary_sum_type;
mod context;
mod dependant_sum_type;
mod dependent_product_type;

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct BinaryProductType {
    left: Box<Type>,
    right: Box<Type>,
}

#[derive(Debug)]
pub enum Type {
    /// universe type
    Type,
    Atom(AtomicType),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        println!("{:?} {:?}", self, other);
        match self {
            Type::Type => matches!(other, Type::Type),
            Type::Atom(atom) => match other {
                Type::Type => false,
                Type::Atom(other_atom) => atom == other_atom,
            },
        }
    }
}

#[derive(Debug)]
enum AtomicType {
    BinaryProductType(BinaryProductType),
    DependentProductType(DependentProductType),
    FunctionType(FunctionType),
    BinarySumTypes(BinarySumType),
    EmptyType(EmptyType),
    DependantSumType(DependantSumType),
    FiniteType(u128),
    Unit,
}

impl PartialEq for AtomicType {
    fn eq(&self, other: &Self) -> bool {
        println!("{:?} {:?}", self, other);
        match (self, other) {
            (AtomicType::Unit, AtomicType::Unit) => true,
            (AtomicType::EmptyType(_), _) | (_, AtomicType::EmptyType(_)) => {
                panic!("found the empty type, something has gone horrid")
            }
            _ => false,
        }
    }
}

// cannot be instanced
#[derive(Debug)]
enum EmptyType {}

// not sure if I've correctly interpreted this. come back to it later.
#[derive(Debug)]
struct FunctionType {
    input: Box<Type>,
    output: Box<Type>,
}

#[derive(Debug)]
pub struct Term {
    r#type: Box<Type>,
    value: String,
}
