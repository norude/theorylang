//! This level gives a unique scope for every binding
//! and captures them for lambda functions
//! It also desugars composition binops
use super::level0;
use crate::common::{Ident, Scope};
use std::collections::HashSet;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State<'a> {
    bindings: Vec<Binding<'a>>, // a stack with global bindings at the bottom
    // OPTIM: make this a HashMap of Stacks instead of a Stack
    captures: Vec<(usize, HashSet<Binding<'a>>)>, // for a binding, whose idx < captures[_].0,
                                                  // its scope should be put in the captures[_].1,.
                                                  // OPTIM: make this a bisect thing based on the order of usize-s
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
    Call,           // a b
    Addition,       // a + b
    Multiplication, // a * b
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
    Number(i32),
    LambdaFunction {
        arg: Binding<'a>,
        body: Box<Self>,
        captured: HashSet<Binding<'a>>,
    },
    BinaryOperation(Box<Self>, BinaryOpKind, Box<Self>),
    Referal {
        name: Ident<'a>,
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

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::LambdaFunction { arg, body, .. } => write!(f, "|{arg}| ({body})"),
            Expr::BinaryOperation(lhs, kind, rhs) => write!(f, "({lhs}{kind}{rhs})"),
            Expr::Referal { name, .. } => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Binding<'a> {
    pub name: Ident<'a>,
    pub scope: Scope,
}

impl std::fmt::Display for Binding<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::hash::Hash for Binding<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.scope.hash(state);
    }
}

impl PartialEq for Binding<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.scope == other.scope
    }
}

impl<'a> State<'a> {
    pub fn map_expr(&mut self, expr: level0::Expr<'a>) -> Expr<'a> {
        match expr {
            level0::Expr::Number(a) => Expr::Number(a),
            level0::Expr::LambdaFunction { arg, body } => {
                self.captures.push((self.bindings.len(), HashSet::new()));
                self.introduce_binding(arg);

                let body = Box::new(self.map_expr(*body));

                let arg = self.bindings.pop().unwrap();
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
                        // a.b -> |point|a(b(point))
                        let point = Ident::unique();
                        self.map_expr(level0::Expr::LambdaFunction {
                            arg: level0::Binding(point),
                            body: Box::new(level0::Expr::BinaryOperation(
                                lhs,
                                level0::BinaryOpKind::Call,
                                Box::new(level0::Expr::BinaryOperation(
                                    rhs,
                                    level0::BinaryOpKind::Call,
                                    Box::new(level0::Expr::Referal(point)),
                                )),
                            )),
                        })
                    }
                }
            }

            level0::Expr::Referal(name) => {
                let (idx, &relevant_binding) = self
                    .bindings
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|b| b.1.name == name)
                    .expect("this binding wasn't found"); // TODO: report this properly
                for (cutoff, set) in &mut self.captures {
                    if idx < *cutoff {
                        set.insert(relevant_binding);
                    }
                }
                Expr::Referal {
                    name,
                    scope: relevant_binding.scope,
                }
            }
        }
    }
    pub fn introduce_binding(&mut self, binding: level0::Binding<'a>) {
        self.bindings.push(Binding {
            name: binding.0,
            scope: Scope::new(),
        });
    }
}
