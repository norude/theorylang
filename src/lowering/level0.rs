use crate::common::Ident;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Binding<'a>(pub Ident<'a>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr<'a> {
    Number(i32),
    LambdaFunction { arg: Binding<'a>, body: Box<Self> },
    Addition(Box<Self>, Box<Self>),
    Multiplication(Box<Self>, Box<Self>),
    Call(Box<Self>, Box<Self>),
    Referal(Ident<'a>),
}
