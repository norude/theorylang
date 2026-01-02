mod args;
mod ast;
mod common;
use crate::args::get_args;
use crate::ast::parser;
use ariadne::{Color, Label, Report, ReportKind, sources};
use chumsky::Parser;
use std::path::Path;

fn report_err(path: &str, text: &str, err: &chumsky::prelude::Rich<'_, char>) {
    Report::build(
        ReportKind::Error,
        (path.to_string(), err.span().into_range()),
    )
    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
    .with_message(err.to_string())
    .with_label(
        Label::new((path.to_string(), err.span().into_range()))
            .with_message(err.reason().to_string())
            .with_color(Color::Red),
    )
    .with_labels(err.contexts().map(|(label, span)| {
        Label::new((path.to_string(), span.into_range()))
            .with_message(format!("while parsing this {label}"))
            .with_color(Color::Yellow)
    }))
    .finish()
    .eprint(sources([(path.to_string(), text)]))
    .unwrap();
}

fn main() -> rustyline::Result<()> {
    let args = get_args();
    if let Some(path) = args.file {
        let text = std::fs::read_to_string(Path::new(&path)).unwrap();
        let Ok(tree) = parser().parse(&text).into_result().map_err(|errs| {
            for err in errs {
                report_err(&path, &text, &err);
            }
        }) else {
            return Ok(());
        };
        println!("{tree}");
        let lowered = tree.lower_all_the_way();
        lowered.eval();
        return Ok(());
    }
    let mut rl = rustyline::DefaultEditor::new()?;
    loop {
        let Ok(line) = rl.readline(">> ") else {
            return Ok(());
        };
        rl.add_history_entry(&line)?;

        let Ok(tree) = parser().parse(&line).into_result().map_err(|errs| {
            for err in errs {
                report_err("stdin", &line, &err);
            }
        }) else {
            continue;
        };

        let lowered = tree.lower_all_the_way();
        lowered.eval();
    }
}
