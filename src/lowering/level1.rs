use super::level0;
use crate::common::Ident;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State();

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
    Number(i32),
    LambdaFunction(LambdaFunction<'a>),
    Addition(Box<Self>, Box<Self>),
    Multiplication(Box<Self>, Box<Self>),
    Call(Box<Self>, Box<Self>),
    Referal(Ident<'a>),
}

pub type Binding<'a> = level0::Binding<'a>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaFunction<'a> {
    pub arg: Binding<'a>,
    pub body: Box<Expr<'a>>,
    // captured: HashMap<Ident<'a>, Value<'a>>,
}

impl<'a> level0::Expr<'a> {
    pub fn map(self, state: &mut State) -> Expr<'a> {
        match self {
            Self::Number(a) => Expr::Number(a),
            Self::LambdaFunction(level0::LambdaFunction { arg, body }) => {
                Expr::LambdaFunction(LambdaFunction {
                    arg,
                    body: Box::new(body.map(state)),
                })
            }
            Self::Addition(a, b) => Expr::Addition(Box::new(a.map(state)), Box::new(b.map(state))),
            Self::Multiplication(a, b) => {
                Expr::Multiplication(Box::new(a.map(state)), Box::new(b.map(state)))
            }
            Self::Call(a, b) => Expr::Call(Box::new(a.map(state)), Box::new(b.map(state))),
            Self::Referal(a) => Expr::Referal(a),
        }
    }
}
