pub(crate) struct Context {
    types: Vec<Type>,
}

enum Type {
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


struct DependentProductType {
    dependant: Box<Term>,
    dependee: Box<Type>,
}

struct Term {}

impl DependentProductType {
    // make sure to typecheck the term
    fn new(context: &Context, family: Box<dyn Fn(Term) -> Type>, term: Term) -> Result<Self, &'static str> {
        todo!()
    }
}