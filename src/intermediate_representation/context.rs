use std::marker::PhantomData;

use crate::intermediate_representation::{AtomicType, Type};

pub struct Context {
    pub types: Vec<Type>,
    // prevents creating via {}
    hidden: PhantomData<()>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            types: vec![Type::Type, Type::Atom(AtomicType::Unit)],
            hidden: Default::default(),
        }
    }
}
