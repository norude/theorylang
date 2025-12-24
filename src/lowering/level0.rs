use crate::common::Ident;

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
