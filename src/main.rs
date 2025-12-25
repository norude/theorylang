mod args;
mod lowering;
mod parser;
mod common {
    // TODO: move this to a better place
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    fn get_id() -> usize {
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Id(usize);
    impl Id {
        pub fn new() -> Self {
            Self(get_id())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Scope(Id);
    impl Scope {
        pub fn new() -> Self {
            Self(Id::new())
        }
    }
    #[derive(Clone, PartialEq, Eq, Copy, Hash)]
    pub enum Ident<'a> {
        Real(&'a str),
        CompilerInserted(Id),
    }

    impl Ident<'_> {
        pub fn unique() -> Self {
            Self::CompilerInserted(Id::new())
        }
    }

    impl std::fmt::Debug for Ident<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self}")
        }
    }
    impl std::fmt::Display for Ident<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Real(s) => write!(f, "{s}"),
                Self::CompilerInserted(id) => write!(f, "#{}", id.0),
            }
        }
    }
}
use crate::args::get_args;
use crate::lowering::Lower;
use crate::parser::parser;
use ariadne::{Color, Label, Report, ReportKind, sources};
use chumsky::Parser;
use std::fs;
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
        let text = fs::read_to_string(Path::new(&path)).unwrap();
        let Ok(tree) = parser().parse(&text).into_result().map_err(|errs| {
            for err in errs {
                report_err(&path, &text, &err);
            }
        }) else {
            return Ok(());
        };
        println!("{tree}");
        let value = tree.lower_all_the_way();
        println!("{value}");
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

        let value = tree.lower_all_the_way();
        println!("{value}");
    }
}
