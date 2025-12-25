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
    pub struct Scope(usize);
    impl Scope {
        pub fn new() -> Self {
            Self(get_id())
        }
    }
    #[derive(Clone, PartialEq, Eq, Copy, Hash)]
    pub struct Ident<'a>(pub &'a str);
    impl std::fmt::Debug for Ident<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
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

fn main() {
    let args = get_args();
    let path = args.file;
    let text = fs::read_to_string(Path::new(&path)).unwrap();
    let tree = match parser().parse(&text).into_result() {
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, (path.clone(), err.span().into_range()))
                    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                    .with_message(err.to_string())
                    .with_label(
                        Label::new((path.clone(), err.span().into_range()))
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .with_labels(err.contexts().map(|(label, span)| {
                        Label::new((path.clone(), span.into_range()))
                            .with_message(format!("while parsing this {label}"))
                            .with_color(Color::Yellow)
                    }))
                    .finish()
                    .eprint(sources([(path.clone(), &text)]))
                    .unwrap();
            }
            return;
        }
        Ok(tree) => tree,
    };
    println!("{tree:?}");
    let value = tree.lower_all_the_way();
    println!("{value:?}");
}
