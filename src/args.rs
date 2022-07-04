use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"), version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    #[clap(help = "File to compile", empty_values = false)]
    pub input: PathBuf,
    #[clap(
        help = "Output path. Defaults to [input path].bin",
        short = 'o',
        long = "--output"
    )]
    pub output: Option<PathBuf>,
}

impl Args {
    pub fn parse() -> Args {
        <Args as Parser>::parse()
    }
}
