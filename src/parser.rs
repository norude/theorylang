#![allow(dead_code)]
use chumsky::prelude::*;

pub struct Ident<'a>(&'a str);

pub struct Binding<'a>(Ident<'a>);
pub enum Expr<'a> {
    Number(i32),
    LambdaFunction {
        args: Vec<Binding<'a>>,
        body: Box<Self>,
    },
    Addition(Box<Self>, Box<Self>),
    Multiplication(Box<Self>, Box<Self>),
    Call(Box<Self>, Box<Self>),
    Referal(Ident<'a>),
}

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
        choice((
            binding()
                .separated_by(op('|'))
                .collect::<Vec<_>>()
                .delimited_by(op('|'), op('|'))
                .then(expression.clone())
                .map(|(args, body)| Expr::LambdaFunction {
                    args,
                    body: Box::new(body),
                }),
            //
            expression.clone().delimited_by(op('('), op(')')),
            //
            number().map(Expr::Number),
            //
            expression.clone().foldl(
                op('*').ignore_then(expression.clone()).repeated(),
                |lhs, rhs| Expr::Multiplication(Box::new(lhs), Box::new(rhs)),
            ),
            expression.clone().foldl(
                op('+').ignore_then(expression.clone()).repeated(),
                |lhs, rhs| Expr::Addition(Box::new(lhs), Box::new(rhs)),
            ),
            //
            expression
                .clone()
                .then(expression.clone())
                .map(|(func, arg)| Expr::Call(Box::new(func), Box::new(arg))),
            //
            ident().map(Expr::Referal),
            //
        ))
        .padded()
    })
}

pub fn parser<'a>() -> parser!('a: Expr<'a>) {
    expression()
}
