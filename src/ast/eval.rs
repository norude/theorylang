use super::level1;
use crate::ast::level0::GlobalSymbol;
use crate::common::Scope;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Number(i32),
    Function {
        arg: level1::Binding,
        body: level1::Expr<'a>,
        captures: HashMap<level1::Binding, Self>,
    },
}

impl std::fmt::Display for Value<'_> {
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
pub struct State<'a> {
    bindings: HashMap<Scope, Value<'a>>,
    globals: HashMap<GlobalSymbol<'a>, level1::Top<'a>>,
}

impl<'s> State<'s> {
    pub fn eval_expr(&mut self, expr: level1::Expr<'s>) -> Value<'s> {
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
                let lhs = self.eval_expr(*lhs);
                let rhs = self.eval_expr(*rhs);
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
                            let res = self.eval_expr(body);
                            self.bindings = old_bindings;
                            res
                        }
                        _ => panic!(),
                    },
                }
            }
            level1::Expr::ProcCall { name, args } => {
                let passed_args = args
                    .into_iter()
                    .map(|x| self.eval_expr(x))
                    .collect::<Vec<_>>();
                let top = self.globals.get(&name).unwrap();
                match top {
                    level1::Top::Procedure {
                        args,
                        body,
                        name: _,
                        return_type: _,
                    } => {
                        let old_bindings = std::mem::take(&mut self.bindings);
                        self.bindings.extend(
                            args.iter()
                                .zip(passed_args)
                                .map(|((name, _type), value)| (name.scope, value)),
                        );
                        let res = self.eval_expr(body.clone());
                        self.bindings = old_bindings;
                        res
                    }
                }
            }
        }
    }
    pub fn eval_top(&mut self, top: level1::Top<'s>) {
        match top {
            level1::Top::Procedure { name, .. } => {
                self.globals.insert(name, top);
            }
        }
    }
}
