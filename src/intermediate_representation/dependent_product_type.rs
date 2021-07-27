use crate::intermediate_representation::context::Context;
use crate::intermediate_representation::{Term, Type};

#[derive(Debug)]
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
