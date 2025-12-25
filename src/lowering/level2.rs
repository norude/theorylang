use std::collections::HashMap;

use crate::common::Scope;
use crate::lowering::level1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Number(i32),
    Function {
        arg: level1::Binding<'a>,
        body: level1::Expr<'a>,
        captures: HashMap<Scope, Self>,
    },
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
                    .map(|s| (s, self.bindings.get(&s).unwrap().clone()))
                    .collect(),
            },
            level1::Expr::Addition(x, y) => {
                let x = self.map_expr(*x);
                let y = self.map_expr(*y);
                match (x, y) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                    _ => panic!(),
                }
            }
            level1::Expr::Multiplication(x, y) => {
                let x = self.map_expr(*x);
                let y = self.map_expr(*y);
                match (x, y) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                    _ => panic!(),
                }
            }
            level1::Expr::Referal { scope, name: _ } => self.bindings.get(&scope).unwrap().clone(),
            level1::Expr::Call(x, y) => {
                let x = self.map_expr(*x);
                let passed = self.map_expr(*y);
                match x {
                    Value::Function {
                        arg,
                        body,
                        captures,
                    } => {
                        self.bindings.insert(arg.scope, passed);
                        let capture_keys = captures.keys().copied().collect::<Vec<_>>();
                        self.bindings.extend(captures);
                        let res = self.map_expr(body);
                        self.bindings.remove(&arg.scope);
                        for scope in capture_keys {
                            self.bindings.remove(&scope);
                        }
                        res
                    }
                    Value::Number(_) => panic!(),
                }
            }
        }
    }
}
