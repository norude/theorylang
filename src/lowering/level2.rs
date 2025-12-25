use std::collections::HashMap;

use crate::common::Scope;
use crate::lowering::level1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Number(i32),
    Function {
        arg: level1::Binding<'a>,
        body: level1::Expr<'a>,
        captures: HashMap<level1::Binding<'a>, Self>,
    },
}

impl std::fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::Function {
                arg,
                body,
                captures,
            } => {
                if !captures.is_empty() {
                    write!(f, "captured(")?;
                    for (idx, (binding, value)) in captures.iter().enumerate() {
                        write!(f, "{}={value}", binding.name)?;
                        if idx != captures.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, ") ")?;
                }
                write!(f, "|{arg}| {body}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State<'a> {
    bindings: HashMap<Scope, Value<'a>>,
}

impl<'a> State<'a> {
    pub fn map_expr(&mut self, expr: level1::Expr<'a>) -> Value<'a> {
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

            level1::Expr::Referal { scope, name: _ } => self.bindings.get(&scope).unwrap().clone(),
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
                            self.bindings.insert(arg.scope, passed);
                            let capture_keys = captures.keys().copied().collect::<Vec<_>>();
                            self.bindings
                                .extend(captures.into_iter().map(|(k, v)| (k.scope, v)));
                            let res = self.map_expr(body);
                            self.bindings.remove(&arg.scope);
                            for scope in capture_keys {
                                self.bindings.remove(&scope.scope);
                            }
                            res
                        }
                        _ => panic!(),
                    },
                }
            }
        }
    }
}
