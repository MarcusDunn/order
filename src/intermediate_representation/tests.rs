use crate::intermediate_representation::context::Context;

use super::*;

#[test]
fn constrct_sum_type() {
    let mut context = Context::default();
    BinarySumType::new(
        &mut context,
        box Type::Atom(AtomicType::Unit),
        box Type::Atom(AtomicType::Unit),
    )
    .unwrap();
}
