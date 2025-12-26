use crate::common::Ident;
use crate::lowering::level0::{BinaryOpKind, Binding, Expr};
use chumsky::prelude::*;

macro_rules! parser {
    ($life:lifetime: $out:ty) => {
        impl Parser <$life, &$life str, $out, extra::Err<Rich<$life,char>>> + Clone
    }
}

macro_rules! keywords {
    ($($kw:literal <= $kw_parser:ident)*) => {
        fn is_kw(s: &str) -> bool {
            $( if s == $kw { return true;} )*
            false
        }
        $(
            fn $kw_parser<'a>() -> parser!('a: ()) {
                just($kw).padded().ignored().labelled($kw)
            }
        )*
    };
}

fn ident<'a>() -> parser!('a: Ident<'a>) {
    any()
        .filter(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'))
        .then(
            any()
                .filter(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))
                .repeated(),
        )
        .to_slice()
        .filter(|&s| !is_kw(s))
        .map(Ident)
        .padded()
        .labelled("identifier")
}

keywords! {
    "let" <= kw_let
    "in" <= kw_in
}

fn number<'a>() -> parser!('a: i32) {
    text::int(10)
        .try_map(|s: &str, span| s.parse().map_err(|e| Rich::custom(span, e)))
        .padded()
        .labelled("number")
}

fn op<'a>(x: &'static str) -> parser!('a: ()) {
    just(x).padded().ignored()
}

fn binding<'a>() -> parser!('a: Binding<'a>) {
    ident().map(Binding).labelled("binding")
}

fn expression<'a>() -> parser!('a: Expr<'a>) {
    recursive(|expression| {
        let lambda = binding()
            .then_ignore(op("->"))
            .then(expression.clone())
            .map(|(arg, body)| Expr::LambdaFunction {
                arg,
                body: Box::new(body),
            });
        let let_binding = kw_let()
            .ignore_then(binding())
            .then_ignore(op("="))
            .then(expression.clone())
            .then_ignore(kw_in())
            .then(expression.clone())
            .map(|((name, value), body)| Expr::LetBinding {
                name,
                value: Box::new(value),
                body: Box::new(body),
            });
        let parenthesised = expression.clone().delimited_by(op("("), op(")"));
        let number = number().map(Expr::Number);
        let referal = ident().map(Expr::Referal);

        let expr = choice((let_binding, lambda, parenthesised, number, referal)).padded();
        let expr = expr
            .clone()
            .foldl(op("*").ignore_then(expr).repeated(), |lhs, rhs| {
                Expr::BinaryOperation(Box::new(lhs), BinaryOpKind::Multiplication, Box::new(rhs))
            });
        let expr = expr
            .clone()
            .foldl(op("+").ignore_then(expr).repeated(), |lhs, rhs| {
                Expr::BinaryOperation(Box::new(lhs), BinaryOpKind::Addition, Box::new(rhs))
            });

        let expr = expr
            .clone()
            .foldl(op(".").ignore_then(expr).repeated(), |lhs, rhs| {
                Expr::BinaryOperation(Box::new(lhs), BinaryOpKind::Composition, Box::new(rhs))
            });

        let expr = expr.clone().foldl(expr.repeated(), |lhs, rhs| {
            Expr::BinaryOperation(Box::new(lhs), BinaryOpKind::Call, Box::new(rhs))
        });

        expr.padded().labelled("expression")
    })
}

pub fn parser<'a>() -> parser!('a: Expr<'a>) {
    expression()
}
