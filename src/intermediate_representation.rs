use crate::intermediate_representation::dependent_product_type::DependentProductType;

pub struct Context {
    types: Vec<Type>,
}

pub enum Type {
    /// universe type
    Type,
    Atom(AtomicType),
}

enum AtomicType {
    BinaryProductType(BinaryProductType),
    DependentProductType(DependentProductType),
    Unit,
}

struct BinaryProductType {
    left: Box<Type>,
    right: Box<Type>,
}

mod dependent_product_type {
    use crate::intermediate_representation::{Context, Term, Type};

    pub struct DependentProductType {
        dependant: Box<Term>,
        dependee: Box<Type>,
    }

    impl DependentProductType {
        // make sure to typecheck the term
        pub fn new(
            context: &Context,
            family: Box<dyn Fn(Term) -> Type>,
            term: Term,
        ) -> Result<Self, &'static str> {
            todo!()
        }
    }
}

pub struct Term {}
