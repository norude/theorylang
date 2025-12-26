use crate::common::Ident;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Binding<'a>(pub Ident<'a>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
    Call,           // a b
    Addition,       // a + b
    Multiplication, // a * b
    Composition,    // a . b
}
impl std::fmt::Display for BinaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Call => write!(f, " "),
            Self::Addition => write!(f, " + "),
            Self::Multiplication => write!(f, " * "),
            Self::Composition => write!(f, " . "),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr<'a> {
    Number(i32),
    LambdaFunction {
        arg: Binding<'a>,
        body: Box<Self>,
    },
    LetBinding {
        name: Binding<'a>,
        value: Box<Self>,
        body: Box<Self>,
    },
    BinaryOperation(Box<Self>, BinaryOpKind, Box<Self>),
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
            Expr::LetBinding {
                name,
                value,
                body: scope,
            } => write!(f, "let {name} = {value} in\n{scope}"),
            Expr::BinaryOperation(lhs, kind, rhs) => write!(f, "({lhs}{kind}{rhs})"),
            Expr::Referal(name) => write!(f, "{name}"),
        }
    }
}
