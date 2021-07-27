use crate::intermediate_representation::context::Context;
use crate::intermediate_representation::{Term, Type};

// also not sure if this is correct
#[derive(Debug)]
pub struct DependantSumType {
    dependant: Box<Term>,
    dependee: Box<Type>,
}

impl DependantSumType {
    fn new(
        context: &Context,
        // A(x)
        family: Box<dyn Fn(Term) -> Type>,

        term: Term,
    ) -> Result<Self, &'static str> {
        todo!()
    }
}
