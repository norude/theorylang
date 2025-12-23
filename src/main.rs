mod args;
mod parser;

use std::fs;
use std::path::Path;

fn main() {
    use args::get_args;
    use chumsky::Parser;
    use parser::parser;
    let args = get_args();
    let text = fs::read_to_string(Path::new(&args.file)).unwrap();
    let parsed = parser().parse(&text);
}
