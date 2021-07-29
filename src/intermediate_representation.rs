use crate::intermediate_representation::InferableTerm::Free;
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq, Clone)]
// term uparrow
enum InferableTerm {
    Annotated(Box<CheckableTerm>, Type),
    Bound(i32),
    Free(Name),
    Application(Box<InferableTerm>, Box<CheckableTerm>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
// term downarrow
enum CheckableTerm {
    Inferable(Box<InferableTerm>),
    Lambda(Box<CheckableTerm>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Name {
    Global(String),
    Local(i32),
    Quote(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Type {
    Free(Name),
    Function(Box<Type>, Box<Type>),
}

enum Value {
    Lambda(Box<dyn Fn(Value) -> Value>),
    Neutral(Box<Neutral>),
}

impl Value {
    fn v_free(n: Name) -> Value {
        Value::Neutral(Box::new(Neutral::Free(n)))
    }

    fn v_app(self, v: Value) -> Value {
        match self {
            Value::Lambda(f) => f(v),
            Value::Neutral(n) => Value::Neutral(Box::new(Neutral::App(n, Box::new(v)))),
        }
    }
}

type Env = Vec<Value>;

enum Neutral {
    Free(Name),
    App(Box<Neutral>, Box<Value>),
}

impl InferableTerm {
    fn eval(self, env: &Env) -> Value {
        match self {
            InferableTerm::Annotated(box e, _) => e.eval(env),
            InferableTerm::Bound(i) => env.get(i as usize).unwrap(),
            InferableTerm::Free(x) => Value::v_free(x),
            InferableTerm::Application(box e, box ep) => e.eval(env).v_app(ep.eval(env)),
        }
    }
}

impl CheckableTerm {
    fn eval(self, env: &Env) -> Value {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
enum Kind {
    Star,
}

#[derive(Debug)]
enum Info {
    HasKind(Kind),
    HasType(Type),
}

struct Context(Vec<(Name, Info)>);

impl Context {
    fn kind(&self, t: &Type, _k: Kind) -> Result<(), String> {
        match t {
            Type::Free(name) => match self.0.iter().find(|(n, i)| name == n) {
                None => Err(String::from("unknown identifier")),
                Some((_, Info::HasKind(Kind::Star))) => Ok(()),
                _ => unimplemented!(),
            },
            Type::Function(box t, box r) => self
                .kind(t, Kind::Star)
                .and_then(|_| self.kind(r, Kind::Star)),
        }
    }

    fn type_0(&mut self, t: InferableTerm) -> Result<Type, String> {
        self.type_ua(0, t)
    }

    fn type_ua(&mut self, i: i32, inferable_term: InferableTerm) -> Result<Type, String> {
        match inferable_term {
            InferableTerm::Annotated(box e, t) => self
                .kind(&t, Kind::Star)
                .and_then(|_| self.type_da(i, e, &t))
                .and_then(|_| Ok(t)),
            InferableTerm::Free(name) => match self.0.iter().find(|(n, _)| n == &name) {
                None => Err(String::from("illegal application")),
                Some((_, Info::HasType(t))) => Ok(t.clone()),
                _ => unimplemented!(),
            },
            InferableTerm::Application(box e, box ep) => {
                self.type_ua(i, e).and_then(|alpha| match alpha {
                    Type::Function(box t, box tp) => self.type_da(i, ep, &t).and_then(|_| Ok(tp)),
                    _ => Err(String::from("unknown identifier")),
                })
            }
            _ => unimplemented!(),
        }
    }

    fn type_da(&mut self, i: i32, checkable_term: CheckableTerm, t: &Type) -> Result<(), String> {
        match (checkable_term, t) {
            (CheckableTerm::Inferable(box e), t) => self.type_ua(i, e).and_then(|tp| {
                if &tp == t {
                    Err(String::from("type mismatch"))
                } else {
                    Ok(())
                }
            }),
            (CheckableTerm::Lambda(box e), Type::Function(box t, box tp)) => {
                self.0.push((Name::Local(i), Info::HasType(t.clone())));
                self.type_da(
                    i + 1,
                    subst_da(0, &InferableTerm::Free(Name::Local(i)), e),
                    tp,
                )
            }
            _ => Err(String::from("type mismatch")),
        }
    }
}
/*
subst↑ :: Int →Term↑ →Term↑ →Term↑
subst↑ i r (Ann e τ)=Ann (subst↓ i r e)τ
subst↑ i r (Bound j)=if i = = j then r else Bound j
subst↑ i r (Free y)=Free y
subst↑ i r (e :@: e′)=subst↑ i r e :@: subst↓ i r e′
 */
fn subst_ua(i: i32, r: &InferableTerm, inferable_term: InferableTerm) -> InferableTerm {
    match inferable_term {
        InferableTerm::Annotated(box e, t) => {
            InferableTerm::Annotated(Box::new(subst_da(i, r, e)), t)
        }
        InferableTerm::Bound(j) => {
            if i == j {
                r.clone()
            } else {
                InferableTerm::Bound(j)
            }
        }
        InferableTerm::Free(y) => Free(y),
        InferableTerm::Application(e, box ep) => subst_ua(
            i,
            r,
            InferableTerm::Application(e, Box::new(subst_da(i, r, ep))),
        ),
    }
}
/*
subst↓ :: Int →Term↑ →Term↓ →Term↓
subst↓ i r (Inf e)=Inf (subst↑ i r e)
subst↓ i r (Lam e)=Lam (subst↓ (i +1)r e)
 */
fn subst_da(
    i: i32,
    inferable_term: &InferableTerm,
    checkable_term: CheckableTerm,
) -> CheckableTerm {
    todo!()
}
