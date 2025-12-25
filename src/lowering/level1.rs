//! This level gives a unique scope for every binding
//! and captures them for lambda functions
//! It also desugars composition binops
use super::level0;
use crate::common::{Ident, Scope};
use std::collections::HashSet;

mod keyed_stack {
    use std::collections::HashMap;
    use std::hash::Hash;
    #[derive(Debug, Clone)]
    pub struct KeyedStack<K, V> {
        stacks: HashMap<K, Vec<(usize, V)>>,
        length: usize,
    }

    impl<K, V> Default for KeyedStack<K, V> {
        fn default() -> Self {
            Self {
                stacks: HashMap::new(),
                length: 0,
            }
        }
    }

    impl<K: Hash + Eq, V> KeyedStack<K, V> {
        pub const fn len(&self) -> usize {
            self.length
        }
        pub fn push(&mut self, key: K, value: V) {
            self.stacks
                .entry(key)
                .or_default()
                .push((self.length, value));
            self.length += 1;
        }
        pub fn pop(&mut self, key: &K) -> Option<V> {
            self.length -= 1;
            self.stacks
                .get_mut(key)
                .and_then(|stack| stack.pop().map(|(_, v)| v))
        }
        pub fn find(&self, key: &K) -> Option<(usize, &V)> {
            self.stacks
                .get(key)
                .and_then(|stack| stack.last().map(|(idx, v)| (*idx, v)))
        }
    }
}
use keyed_stack::KeyedStack;

#[derive(Debug, Default, Clone)]
pub struct State<'a> {
    bindings: KeyedStack<Ident<'a>, Binding>, // this is a stack
    // with the most global bindings at the bottom.
    // It's keyed because most of the time, Idents are diffrent
    captures: Vec<(usize, HashSet<Binding>)>, // for a binding, whose idx < captures[_].0,
                                              // its scope should be put in the captures[_].1,.
                                              // should be sorted by the .0
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
    Call,           // a b
    Addition,       // a + b
    Multiplication, // a * b
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(i32),
    LambdaFunction {
        arg: Binding,
        body: Box<Self>,
        captured: HashSet<Binding>,
    },
    BinaryOperation(Box<Self>, BinaryOpKind, Box<Self>),
    Referal {
        scope: Scope,
    },
}

impl std::fmt::Display for BinaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Call => write!(f, " "),
            Self::Addition => write!(f, " + "),
            Self::Multiplication => write!(f, " * "),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::LambdaFunction { arg, body, .. } => write!(f, "|{arg}| ({body})"),
            Self::BinaryOperation(lhs, kind, rhs) => write!(f, "({lhs}{kind}{rhs})"),
            Self::Referal { scope } => write!(f, "{scope}"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Binding {
    pub scope: Scope,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.scope)
    }
}

impl std::hash::Hash for Binding {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.scope.hash(state);
    }
}

impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.scope == other.scope
    }
}

impl<'a> State<'a> {
    pub fn map_expr(&mut self, expr: level0::Expr<'a>) -> Expr {
        match expr {
            level0::Expr::Number(a) => Expr::Number(a),
            level0::Expr::LambdaFunction { arg, body } => {
                self.captures.push((self.bindings.len(), HashSet::new()));
                let key = self.introduce_binding(arg);

                let body = Box::new(self.map_expr(*body));

                let arg = self.bindings.pop(&key).unwrap();
                let (_, captured) = self.captures.pop().unwrap();

                Expr::LambdaFunction {
                    arg,
                    body,
                    captured,
                }
            }

            level0::Expr::BinaryOperation(lhs, kind, rhs) => {
                macro_rules! simple {
                    ($op:ident) => {
                        Expr::BinaryOperation(
                            Box::new(self.map_expr(*lhs)),
                            BinaryOpKind::$op,
                            Box::new(self.map_expr(*rhs)),
                        )
                    };
                }
                match kind {
                    level0::BinaryOpKind::Call => simple!(Call),
                    level0::BinaryOpKind::Addition => simple!(Addition),
                    level0::BinaryOpKind::Multiplication => simple!(Multiplication),
                    level0::BinaryOpKind::Composition => {
                        // a.b -> |arg|a(b(point))
                        let arg = Binding {
                            scope: Scope::new(),
                        };

                        self.captures.push((self.bindings.len(), HashSet::new()));
                        let l = self.map_expr(*lhs);
                        let r = self.map_expr(*rhs);
                        let (_, captured) = self.captures.pop().unwrap();

                        Expr::LambdaFunction {
                            captured,
                            arg,
                            body: Box::new(Expr::BinaryOperation(
                                Box::new(l),
                                BinaryOpKind::Call,
                                Box::new(Expr::BinaryOperation(
                                    Box::new(r),
                                    BinaryOpKind::Call,
                                    Box::new(Expr::Referal { scope: arg.scope }),
                                )),
                            )),
                        }
                    }
                }
            }

            level0::Expr::Referal(name) => {
                let (idx, &relevant_binding) = self
                    .bindings
                    .find(&name)
                    .expect("this binding wasn't found"); // TODO: report this properly

                let first_valid = self
                    .captures
                    .binary_search_by_key(&idx, |(cutoff, _)| *cutoff)
                    .map_or_else(|i| i, |i| i + 1);
                for (_, set) in self.captures.iter_mut().skip(first_valid) {
                    set.insert(relevant_binding);
                }
                Expr::Referal {
                    scope: relevant_binding.scope,
                }
            }
        }
    }
    pub fn introduce_binding(&mut self, binding: level0::Binding<'a>) -> Ident<'a> {
        self.bindings.push(
            binding.0,
            Binding {
                scope: Scope::new(),
            },
        );
        binding.0
    }
}
