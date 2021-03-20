mod argument;
mod instruction;
mod parser;

use crate::{instruction::Instruction, parser::InstructionParser};
use clap::{
    app_from_crate,
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    Arg,
};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};

macro_rules! die {
    ($msg:literal) => {{
        eprintln!(concat!("ERROR: ", $msg));
        process::exit(1);
    }};
    ($msg:expr) => {{
        eprintln!("ERROR: {}", $msg);
        process::exit(1);
    }};
}

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("--output")
                .short("o")
                .multiple(true)
                .required(false)
                .help(
                "Output paths. Each argument to -o applies to the INPUT argument with the same index."
                )
        )
        .arg(
            Arg::with_name("INPUT")
                    .multiple(true)
                    .required(true)
                    .help("Input files to compile"),
            ).get_matches();
    let targets: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let mut output_paths: Vec<Option<PathBuf>> = matches
        .values_of("o")
        .unwrap_or(clap::Values::default())
        .map(|s| Some(PathBuf::from(s)))
        .collect();
    match output_paths.len() {
        n if n < targets.len() => output_paths.resize_with(targets.len(), || None),
        n if n > targets.len() => die!("More output paths provided than input files"),
        _ => {}
    };
    for (file, output) in targets.iter().zip(output_paths) {
        let text = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => die!(e),
        };
        let instructions = match InstructionParser::parse_buffer(&text) {
            Ok(s) => s,
            Err(e) => die!(e),
        };
        match write_instructions(
            &instructions,
            output.unwrap_or(gen_output_path(file)).as_path(),
        ) {
            Ok(()) => {}
            Err(e) => die!(e),
        }
    }
}

fn gen_output_path(input_path: &str) -> PathBuf {
    let buf = PathBuf::from(input_path);
    let base = buf.as_path().file_stem().unwrap_or(buf.as_os_str());
    let mut out = PathBuf::from(base);
    out.push(".ch8");
    out
}

fn write_instructions(buf: &[Instruction], path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    let bytes: Vec<u8> = buf.iter().map(Instruction::as_bytes).fold(
        Vec::with_capacity(buf.len()),
        |mut vec, (b0, b1)| {
            vec.push(b0);
            vec.push(b1);
            vec
        },
    );
    let written = file.write(&bytes)?;
    if written < bytes.len() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Attempted to write buffer of {} bytes, but could only write {}",
                bytes.len(),
                written
            ),
        ));
    }
    Ok(())
}
