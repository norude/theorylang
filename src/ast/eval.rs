#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Number(i32),
    Function {
        definition: LambdaFunction<'a>,
        // captured:Scope<Ident<'a>,Value<'a>>, // FIXME:
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluator<'a> {
    scope: Scope<Ident<'a>, Value<'a>>,
}

impl Evaluator<'_> {
    pub fn new() -> Self {
        Self {
            scope: Scope::default(),
        }
    }
}

use crate::ast::scope::Scope;

use super::{Expr, Ident, LambdaFunction, Node};

impl<'a> Node for Expr<'a> {
    type Next = Value<'a>;
    type State = Evaluator<'a>;
    fn map(self, state: &mut Evaluator<'a>) -> Value<'a> {
        match self {
            Expr::Number(x) => Value::Number(x),
            Expr::LambdaFunction(x) => Value::Function { definition: x },
            Expr::Addition(x, y) => {
                let x = x.map(state);
                let y = y.map(state);
                match (x, y) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                    _ => panic!(),
                }
            }
            Expr::Multiplication(x, y) => {
                let x = x.map(state);
                let y = y.map(state);
                match (x, y) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                    _ => panic!(),
                }
            }
            Expr::Referal(x) => dbg!(&state.scope).get(dbg!(&x)).unwrap().clone(),
            Expr::Call(x, y) => {
                let x = x.map(state);
                let y = y.map(state);
                match x {
                    Value::Function {
                        definition: LambdaFunction { arg, body },
                    } => {
                        state.scope.new_scope();
                        dbg!(&state.scope);
                        dbg!(arg.0, &y);
                        state.scope.insert(arg.0, y);
                        let res = body.map(state);
                        state.scope.last_scope();
                        res
                    }
                    Value::Number(_) => panic!(),
                }
            }
        }
    }
}
