use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(help = "File to compile", empty_values = false)]
    pub input: PathBuf,
    #[clap(
        help = "Output path. Defaults to [input path].bin",
        short = 'o',
        long = "--output"
    )]
    pub output: PathBuf,
}

impl Args {
    pub fn parse() -> Args {
        <Args as Parser>::parse()
    }
}
