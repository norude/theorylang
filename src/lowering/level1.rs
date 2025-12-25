use std::collections::HashSet;

/// This level gives a unique scope for every binding
/// and captures them for lambda functions
use super::level0;
use crate::common::{Ident, Scope};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State<'a> {
    bindings: Vec<Binding<'a>>, // a stack with global bindings at the bottom
    // OPTIM: make this a HashMap of Stacks instead of a Stack
    captures: Vec<(usize, HashSet<Scope>)>, // for a binding, whose idx < captures[_].0,
                                            // its scope should be put in the captures[_].1,.
                                            // OPTIM: make this a bisect thing based on the order of usize-s
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
    Number(i32),
    LambdaFunction {
        arg: Binding<'a>,
        body: Box<Self>,
        captured: HashSet<Scope>,
    },
    Addition(Box<Self>, Box<Self>),
    Multiplication(Box<Self>, Box<Self>),
    Call(Box<Self>, Box<Self>),
    Referal {
        name: Ident<'a>,
        scope: Scope,
    },
}

#[derive(Debug, Clone, Eq)]
pub struct Binding<'a> {
    pub name: Ident<'a>,
    pub scope: Scope,
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
            level0::Expr::Addition(a, b) => {
                Expr::Addition(Box::new(self.map_expr(*a)), Box::new(self.map_expr(*b)))
            }
            level0::Expr::Multiplication(a, b) => {
                Expr::Multiplication(Box::new(self.map_expr(*a)), Box::new(self.map_expr(*b)))
            }
            level0::Expr::Call(a, b) => {
                Expr::Call(Box::new(self.map_expr(*a)), Box::new(self.map_expr(*b)))
            }
            level0::Expr::Referal(name) => {
                let (idx, &Binding { name, scope }) = self
                    .bindings
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|b| b.1.name == name)
                    .expect("this binding wasn't found"); // TODO: report this properly
                for (cutoff, set) in &mut self.captures {
                    if idx < *cutoff {
                        set.insert(scope);
                    }
                }
                Expr::Referal { name, scope }
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
