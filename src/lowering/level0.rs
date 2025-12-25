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

impl std::fmt::Display for Binding<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::LambdaFunction { arg, body } => write!(f, "|{arg}| ({body})"),
            Expr::Addition(a, b) => write!(f, "({a} + {b})"),
            Expr::Multiplication(a, b) => write!(f, "({a} * {b})"),
            Expr::Call(a, b) => write!(f, "({a} {b})"),
            Expr::Referal(name) => write!(f, "{name}"),
        }
    }
}
