use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub file: String,
}
pub fn get_args() -> Args {
    Args::parse()
}
