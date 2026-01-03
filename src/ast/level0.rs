use crate::common::Ident;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Binding<'a>(pub Ident<'a>); // equivalent of a rust pattern, so will grow

impl std::fmt::Display for Binding<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct GlobalSymbol<'a>(pub Ident<'a>); // maybe will grow with a mod or crate prefix, will be
// imported, so can't depend on global state and can't have an Id ever

impl std::fmt::Display for GlobalSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Type {
    Never, // zero values
    #[default]
    Unit, // one value
    I32,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::I32 => write!(f, "i32"),
            Self::Unit => write!(f, "()"),
            Self::Never => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Top<'a> {
    Procedure {
        name: GlobalSymbol<'a>,
        args: Vec<(Binding<'a>, Type)>,
        return_type: Type,
        body: Expr<'a>,
    },
}

impl std::fmt::Display for Top<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Procedure {
                name,
                args,
                return_type,
                body,
            } => {
                write!(f, "proc {name}(")?;
                for (idx, (binding, typ)) in args.iter().enumerate() {
                    write!(f, "{binding}:{typ}")?;
                    if idx < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ") -> {return_type} {{\n{body:indent$}\n}}", indent = 4)
            }
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
    ProcCall {
        name: GlobalSymbol<'a>,
        args: Vec<Self>,
    },
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = f.width().unwrap_or(0);
        write!(f, "{:indent$}", "")?;
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::LambdaFunction { arg, body } => write!(f, "{arg} -> {body}"),
            Expr::LetBinding {
                name,
                value,
                body: scope,
            } => write!(f, "let {name} = {value} in\n{scope:indent$}",),
            Expr::BinaryOperation(lhs, kind, rhs) => write!(f, "({lhs}{kind}{rhs})"),
            Expr::Referal(name) => write!(f, "{name}"),
            Expr::ProcCall { name, args } => {
                write!(f, "{name}!(")?;
                for (idx, arg) in args.iter().enumerate() {
                    write!(f, "{arg}")?;
                    if idx < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
    Call,           // a b or b>a
    Addition,       // a + b
    Multiplication, // a * b
    Composition,    // a & b
}

impl std::fmt::Display for BinaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Call => write!(f, " "),
            Self::Addition => write!(f, " + "),
            Self::Multiplication => write!(f, " * "),
            Self::Composition => write!(f, " & "),
        }
    }
}
