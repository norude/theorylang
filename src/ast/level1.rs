//! This level gives a unique scope for every binding
//! and captures them for lambda functions
//! It also desugars composition binops and let bindings
mod keyed_stack;
use super::level0;
use crate::common::{Ident, Scope};
use keyed_stack::KeyedStack;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct State<'a> {
    bindings: KeyedStack<Ident<'a>, Binding>, // this is a stack
    // with the most global bindings at the bottom.
    // It's keyed because most of the time, Idents are diffrent
    captures: Vec<(usize, HashSet<Binding>)>, // for a binding, whose idx < captures[_].0,
    // its scope should be put in the captures[_].1,.
    // should be sorted by the .0
    globals: HashMap<GlobalSymbol<'a>, Top<'a>>, // isn't captured
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
    Call,           // a b
    Addition,       // a + b
    Multiplication, // a * b
}

impl std::fmt::Display for BinaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Call => write!(f, " "),
            Self::Addition => write!(f, " + "),
            Self::Multiplication => write!(f, " * "),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
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
    ProcCall {
        name: GlobalSymbol<'a>,
        args: Vec<Self>,
    },
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::LambdaFunction { arg, body, .. } => write!(f, "{arg} -> {body}"),
            Self::BinaryOperation(lhs, kind, rhs) => write!(f, "({lhs}{kind}{rhs})"),
            Self::Referal { scope } => write!(f, "{scope}"),
            Self::ProcCall { name, args } => {
                write!(f, "{name}!(")?;
                for (idx, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if idx < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

type Type = level0::Type;
type GlobalSymbol<'a> = level0::GlobalSymbol<'a>;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Top<'a> {
    Procedure {
        name: GlobalSymbol<'a>,
        args: Vec<(Binding, Type)>,
        return_type: Type,
        body: Expr<'a>,
    },
}

impl<'a> State<'a> {
    fn introduce_new_binding_in<T>(
        &mut self,
        b: level0::Binding<'a>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> (T, Binding) {
        self.bindings.push(
            b.0,
            Binding {
                scope: Scope::new(),
            },
        );
        (f(self), self.bindings.pop(&b.0).unwrap())
    }
    fn introduce_new_bindings_in<T, X>(
        &mut self,
        b: Vec<(level0::Binding<'a>, X)>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> (T, impl Iterator<Item = (Binding, X)>) {
        for b in &b {
            self.bindings.push(
                b.0.0,
                Binding {
                    scope: Scope::new(),
                },
            );
        }
        (
            f(self),
            b.into_iter()
                .map(|(b, x)| (self.bindings.pop(&b.0).unwrap(), x)),
        )
    }

    fn construct_a_function_in<'b>(
        &mut self,
        f: impl FnOnce(&mut Self) -> (Expr<'b>, Binding),
    ) -> Expr<'b> {
        self.captures.push((self.bindings.len(), HashSet::new()));
        let (body, arg) = f(self);
        let (_, captured) = self.captures.pop().unwrap();
        Expr::LambdaFunction {
            arg,
            body: Box::new(body),
            captured,
        }
    }

    pub fn map_expr(&mut self, expr: level0::Expr<'a>) -> Expr<'a> {
        match expr {
            level0::Expr::Number(a) => Expr::Number(a),
            level0::Expr::LambdaFunction { arg, body } => self.construct_a_function_in(|this| {
                this.introduce_new_binding_in(arg, |this| this.map_expr(*body))
            }),
            level0::Expr::LetBinding { name, value, body } => {
                // let name = value in scope -> (|name|body)(value)
                let value = self.map_expr(*value);
                let fun = self.construct_a_function_in(|this| {
                    this.introduce_new_binding_in(name, |this| this.map_expr(*body))
                });

                Expr::BinaryOperation(Box::new(fun), BinaryOpKind::Call, Box::new(value))
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
                        // a.b -> |scope| a(b(scope))
                        let scope = Scope::new();
                        self.construct_a_function_in(|this| {
                            (
                                Expr::BinaryOperation(
                                    Box::new(this.map_expr(*lhs)),
                                    BinaryOpKind::Call,
                                    Box::new(Expr::BinaryOperation(
                                        Box::new(this.map_expr(*rhs)),
                                        BinaryOpKind::Call,
                                        Box::new(Expr::Referal { scope }),
                                    )),
                                ),
                                Binding { scope },
                            )
                        })
                    }
                }
            }

            level0::Expr::Referal(name) => {
                let Some((idx, &relevant_binding)) = self.bindings.find(&name) else {
                    panic!("that binding wasn't found") // TODO: report it properly
                };
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

            level0::Expr::ProcCall { name, args } => {
                let args = args
                    .into_iter()
                    .map(|arg| self.map_expr(arg))
                    .collect::<Vec<_>>();
                let _top = self.globals.get(&name).expect("that proc wasn't found");
                Expr::ProcCall { name, args }
            }
        }
    }

    pub fn map_top(&mut self, top: level0::Top<'a>) -> Top<'a> {
        match top {
            level0::Top::Procedure {
                name,
                args,
                return_type,
                body,
            } => {
                let (body, args) = self.introduce_new_bindings_in(args, |this| this.map_expr(body));

                Top::Procedure {
                    name,
                    args: args.collect(),
                    return_type,
                    body,
                }
            }
        }
    }
}
