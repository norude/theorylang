use std::fmt::Debug;
#[derive(Clone, PartialEq, Eq, Copy, Hash)]
pub struct Ident<'a>(pub &'a str);

impl Debug for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Binding<'a>(pub Ident<'a>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaFunction<'a> {
    pub arg: Binding<'a>,
    pub body: Box<Expr<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr<'a> {
    Number(i32),
    LambdaFunction(LambdaFunction<'a>),
    Addition(Box<Self>, Box<Self>),
    Multiplication(Box<Self>, Box<Self>),
    Call(Box<Self>, Box<Self>),
    Referal(Ident<'a>),
}
pub mod scope;

pub mod eval;

pub trait Node {
    type Next;
    type State;
    fn map(self, state: &mut Self::State) -> Self::Next;
}


