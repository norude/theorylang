use std::collections::HashMap;

use crate::common::Scope;
use crate::lowering::level1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Number(i32),
    Function {
        arg: level1::Binding,
        body: level1::Expr,
        captures: HashMap<level1::Binding, Self>,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Function {
                arg,
                body,
                captures,
            } => {
                write!(f, "{arg} -> ")?;
                for (binding, value) in captures {
                    write!(f, "let {binding}={value} in ")?;
                }
                write!(f, "{body}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    bindings: HashMap<Scope, Value>,
}

impl State {
    pub fn map_expr(&mut self, expr: level1::Expr) -> Value {
        match expr {
            level1::Expr::Number(x) => Value::Number(x),
            level1::Expr::LambdaFunction {
                arg,
                body,
                captured,
            } => Value::Function {
                arg,
                body: *body,
                captures: captured
                    .into_iter()
                    .map(|s| (s, self.bindings.get(&s.scope).unwrap().clone()))
                    .collect(),
            },

            level1::Expr::Referal { scope } => self.bindings.get(&scope).unwrap().clone(),
            level1::Expr::BinaryOperation(lhs, kind, rhs) => {
                use level1::BinaryOpKind as Op;
                let lhs = self.map_expr(*lhs);
                let rhs = self.map_expr(*rhs);
                match kind {
                    Op::Addition => match (lhs, rhs) {
                        (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                        _ => panic!(),
                    },
                    Op::Multiplication => match (lhs, rhs) {
                        (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                        _ => panic!(),
                    },
                    Op::Call => match (lhs, rhs) {
                        (
                            Value::Function {
                                arg,
                                body,
                                captures,
                            },
                            passed,
                        ) => {
                            // Functions are pure. Refering to stuff from the outer scope should be
                            // done with capturing! For that reason, we empty out self.bindings
                            let old_bindings = std::mem::take(&mut self.bindings);
                            self.bindings.insert(arg.scope, passed);
                            self.bindings
                                .extend(captures.into_iter().map(|(k, v)| (k.scope, v)));
                            let res = self.map_expr(body);
                            self.bindings = old_bindings;
                            res
                        }
                        _ => panic!(),
                    },
                }
            }
        }
    }
}
