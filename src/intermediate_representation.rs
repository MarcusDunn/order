#[derive(Debug, Eq, PartialEq, Clone)]
// term uparrow
enum TermUa {
    Annotated(Box<TermDa>, Type),
    Bound(i32),
    Free(Name),
    Application(Box<TermUa>, Box<TermDa>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
// term downarrow
enum TermDa {
    Inferable(Box<TermUa>),
    Lambda(Box<TermDa>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Name {
    Global(String),
    Local(i32),
    Quote(i32),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Type {
    Free(Name),
    Function(Box<Type>, Box<Type>),
}

#[derive(Clone)]
enum Value {
    Lambda(Box<fn(Value) -> Value>),
    Neutral(Box<Neutral>),
}

impl Value {
    fn v_free(n: Name) -> Value {
        Value::Neutral(box Neutral::Free(n))
    }

    fn v_app(self, v: Value) -> Value {
        match self {
            Value::Lambda(f) => f(v),
            Value::Neutral(n) => Value::Neutral(box Neutral::App(n, box v)),
        }
    }
}

#[derive(Clone)]
enum Neutral {
    Free(Name),
    App(Box<Neutral>, Box<Value>),
}

impl TermUa {
    fn eval(self, env: &[Value]) -> Value {
        match self {
            TermUa::Annotated(box e, _) => e.eval(env),
            TermUa::Bound(i) => env.get(i as usize).unwrap().clone(),
            TermUa::Free(x) => Value::v_free(x),
            TermUa::Application(box e, box ep) => e.eval(env).v_app(ep.eval(env)),
        }
    }
}

impl TermDa {
    fn eval(self, env: &[Value]) -> Value {
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

    fn type_0(&mut self, t: TermUa) -> Result<Type, String> {
        self.type_ua(0, t)
    }

    fn type_ua(&mut self, i: i32, inferable_term: TermUa) -> Result<Type, String> {
        match inferable_term {
            TermUa::Annotated(box e, t) => self
                .kind(&t, Kind::Star)
                .and_then(|_| self.type_da(i, e, &t))
                .map(|_| t),
            TermUa::Free(name) => match self.0.iter().find(|(n, _)| n == &name) {
                None => Err(String::from("illegal application")),
                Some((_, Info::HasType(t))) => Ok(t.clone()),
                _ => unimplemented!(),
            },
            TermUa::Application(box e, box ep) => {
                self.type_ua(i, e).and_then(|alpha| match alpha {
                    Type::Function(box t, box tp) => self.type_da(i, ep, &t).map(|_| tp),
                    _ => Err(String::from("unknown identifier")),
                })
            }
            _ => unimplemented!(),
        }
    }

    fn type_da(&mut self, i: i32, checkable_term: TermDa, t: &Type) -> Result<(), String> {
        match (checkable_term, t) {
            (TermDa::Inferable(box e), t) => self.type_ua(i, e).and_then(|tp| {
                if &tp == t {
                    Err(String::from("type mismatch"))
                } else {
                    Ok(())
                }
            }),
            (TermDa::Lambda(box e), Type::Function(box t, box tp)) => {
                self.0.push((Name::Local(i), Info::HasType(t.clone())));
                self.type_da(i + 1, subst_da(0, &TermUa::Free(Name::Local(i)), e), tp)
            }
            _ => Err(String::from("type mismatch")),
        }
    }
}

fn subst_ua(i: i32, r: &TermUa, inferable_term: TermUa) -> TermUa {
    match inferable_term {
        TermUa::Annotated(box e, t) => TermUa::Annotated(box subst_da(i, r, e), t),
        TermUa::Bound(j) => {
            if i == j {
                r.clone()
            } else {
                TermUa::Bound(j)
            }
        }
        TermUa::Free(y) => TermUa::Free(y),
        TermUa::Application(e, box ep) => {
            subst_ua(i, r, TermUa::Application(e, box subst_da(i, r, ep)))
        }
    }
}

/*
subst↓ :: Int →Term↑ →Term↓ →Term↓
subst↓ i r (Inf e)=Inf (subst↑ i r e)
subst↓ i r (Lam e)=Lam (subst↓ (i +1)r e)
 */
fn subst_da(i: i32, inferable_term: &TermUa, checkable_term: TermDa) -> TermDa {
    match checkable_term {
        TermDa::Inferable(box e) => TermDa::Inferable(box subst_ua(i, inferable_term, e)),
        TermDa::Lambda(box e) => TermDa::Lambda(box subst_da(i + 1, inferable_term, e)),
    }
}

/*
quote0 :: Value →Term↓
quote0 =quote 0
*/

fn quote0(v: Value) -> TermDa {
    quote(0, v)
}

/*
quote :: Int →Value →Term↓
quote i (VLam f)=Lam (quote (i +1)(f (vfree (Quote i))))
quote i (VNeutral n)=Inf (neutralQuote i n)
*/
fn quote(i: i32, v: Value) -> TermDa {
    match v {
        Value::Lambda(f) => quote(i + 1, f(Value::v_free(Name::Quote(i)))),
        Value::Neutral(box n) => TermDa::Inferable(box neutral_quote(i, n)),
    }
}

/*
neutralQuote :: Int →Neutral →Term↑
neutralQuote i (NFree x)=boundfree i x
neutralQuote i (NApp n v)=neutralQuote i n :@: quote i v
 */

fn neutral_quote(i: i32, n: Neutral) -> TermUa {
    match n {
        Neutral::Free(x) => bound_free(i, x),
        Neutral::App(box n, box v) => TermUa::Application(box neutral_quote(i, n), box quote(i, v)),
    }
}

/*
boundfree :: Int →Name →Term↑
boundfree i (Quote k)=Bound (i −k −1)
boundfree i x =Free x
 */
fn bound_free(i: i32, n: Name) -> TermUa {
    match n {
        Name::Quote(k) => TermUa::Bound(i - k - 1),
        x => TermUa::Free(x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_id() {
        // id′ = Lam (Inf (Bound 0))
        let _id = TermDa::Lambda(box TermDa::Inferable(box TermUa::Bound(0)));
    }

    #[test]
    fn create_const() {
        // const′ =Lam (Lam (Inf (Bound 1)))
        let _const = TermDa::Lambda(box TermDa::Lambda(box TermDa::Inferable(
            box TermUa::Bound(1),
        )));
    }

    #[test]
    fn create_t_free() {
        // tfree α=TFree (Global α)
        fn _t_free(a: String) -> Type {
            Type::Free(Name::Global(a))
        }
    }

    #[test]
    fn create_free() {
        fn _free(x: String) -> TermDa {
            TermDa::Inferable(box TermUa::Free(Name::Global(x)))
        }

        // free x =Inf (Free (Global x))
    }

    // quote0 (eval↑ term1 [ ])

    // term1 =Ann id′ (Fun (tfree "a")(tfree "a")):@: free "y"
    // term2 =Ann const′ (Fun (Fun (tfree "b")(tfree "b"))
    // (Fun (tfree "a")
    // (Fun (tfree "b")(tfree "b"))))
    // :@: id′ :@: free "y"
    // env1 =[(Global "y",HasType (tfree "a")),
    // (Global "a",HasKind Star)]
    // env2 =[(Global "b",HasKind Star)] ++env1
}
