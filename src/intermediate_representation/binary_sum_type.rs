use crate::intermediate_representation::context::Context;
use crate::intermediate_representation::AtomicType::BinarySumTypes;
use crate::intermediate_representation::{AtomicType, Type};

#[derive(Debug)]
pub struct BinarySumType {
    left: Box<Type>,
    right: Box<Type>,
}

impl BinarySumType {
    pub fn new(context: &mut Context, left: Box<Type>, right: Box<Type>) -> Result<&Self, String> {
        if context.types.contains(&*right) {
            if context.types.contains(&*left) {
                let as_atom = Type::Atom(AtomicType::BinarySumTypes(BinarySumType { left, right }));
                if context.types.contains(&as_atom) {
                    if let Some(Type::Atom(AtomicType::BinarySumTypes(bst))) =
                        context.types.iter().find(|&it| it == &as_atom)
                    {
                        Ok(bst)
                    } else {
                        println!("{:?}", context.types);
                        println!("{:?}", as_atom);
                        unreachable!()
                    }
                } else {
                    context.types.push(as_atom);
                    if let Some(Type::Atom(AtomicType::BinarySumTypes(bst))) = context.types.last()
                    {
                        Ok(bst)
                    } else {
                        unreachable!()
                    }
                }
            } else {
                Err(format!("could not construct binary_sum_type as the left {:?}, does not exist in the current context", left))
            }
        } else {
            Err(format!("could not construct binary_sum_type as the right {:?}, does not exist in the current context", right))
        }
    }
}
