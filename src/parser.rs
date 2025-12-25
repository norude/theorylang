use crate::common::Ident;
use crate::lowering::level0::{Binding, Expr};
use chumsky::prelude::*;

macro_rules! parser {
    ($life:lifetime: $out:ty) => {
        impl Parser <$life, &$life str, $out, extra::Err<Rich<$life,char>>> + Clone
    }
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
        .map(Ident)
        .padded()
        .labelled("identifier")
}

fn number<'a>() -> parser!('a: i32) {
    text::int(10)
        .try_map(|s: &str, span| s.parse().map_err(|e| Rich::custom(span, e)))
        .padded()
}

fn op<'a>(x: char) -> parser!('a: ()) {
    just(x).padded().ignored()
}

fn binding<'a>() -> parser!('a: Binding<'a>) {
    ident().map(Binding)
}

fn expression<'a>() -> parser!('a: Expr<'a>) {
    recursive(|expression| {
        let lambda = binding()
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(op('|'), op('|'))
            .then(expression.clone())
            .map(|(args, body)| {
                args.into_iter()
                    .rev()
                    .fold(body, |body, arg| Expr::LambdaFunction {
                        arg,
                        body: Box::new(body),
                    })
            });
        let parenthesised = expression.clone().delimited_by(op('('), op(')'));
        let number = number().map(Expr::Number);
        let referal = ident().map(Expr::Referal);

        let expr = choice((number, referal, parenthesised, lambda)).padded();
        let expr = expr
            .clone()
            .foldl(op('*').ignore_then(expr).repeated(), |lhs, rhs| {
                Expr::Multiplication(Box::new(lhs), Box::new(rhs))
            });
        let expr = expr
            .clone()
            .foldl(op('+').ignore_then(expr).repeated(), |lhs, rhs| {
                Expr::Addition(Box::new(lhs), Box::new(rhs))
            });
        let expr = expr.clone().foldl(expr.repeated(), |lhs, rhs| {
            Expr::Call(Box::new(lhs), Box::new(rhs))
        });

        expr.padded().labelled("expression")
    })
}

pub fn parser<'a>() -> parser!('a: Expr<'a>) {
    expression()
}
