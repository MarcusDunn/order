struct Context {
    types: Vec<Type>,
}

enum Type {
    Universe(u128),
    Atom(AtomicType),
}

struct AtomicType {
    type_id: u128,
    name: String,
}