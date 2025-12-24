use crate::common::Ident;
use crate::lowering::level1;
use crate::scope::Scope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'a> {
    Number(i32),
    Function {
        definition: level1::LambdaFunction<'a>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State<'a> {
    scope: Scope<Ident<'a>, Value<'a>>,
}

impl<'a> level1::Expr<'a> {
    pub fn map(self, state: &mut State<'a>) -> Value<'a> {
        match self {
            level1::Expr::Number(x) => Value::Number(x),
            level1::Expr::LambdaFunction(x) => Value::Function { definition: x },
            level1::Expr::Addition(x, y) => {
                let x = x.map(&mut *state);
                let y = y.map(state);
                match (x, y) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                    _ => panic!(),
                }
            }
            level1::Expr::Multiplication(x, y) => {
                let x = x.map(state);
                let y = y.map(state);
                match (x, y) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                    _ => panic!(),
                }
            }
            level1::Expr::Referal(x) => state.scope.get(&x).unwrap().clone(),
            level1::Expr::Call(x, y) => {
                let x = x.map(state);
                let y = y.map(state);
                match x {
                    Value::Function {
                        definition: level1::LambdaFunction { arg, body },
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
