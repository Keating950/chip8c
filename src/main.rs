#![allow(unused)]

mod instruction;
mod parser;
use crate::{instruction::Instruction, parser::InstructionParser};
use clap::{
    app_from_crate,
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    values_t,
    App,
    Arg,
};
use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{self, Write},
    mem,
    path::{Path, PathBuf},
    process,
};

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

macro_rules! die {
    ($msg:literal) => {{
        eprintln!($msg);
        process::exit(1);
    }};
    ($msg:expr) => {{
        eprintln!("{}", $msg);
        process::exit(1);
    }};
}

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("o")
                .multiple(true)
                .required(false)
                .help(concat!(
                "Outputs paths for input files. Because only single files are supported as compilation units,",
                " Each argument passed to -o applies to the INPUT argument with the same index."))
        )
        .arg(
            Arg::with_name("INPUT")
                    .multiple(true)
                    .required(true)
                    .help("Input files to compile"),
            );
    let matches = app.get_matches();
    let targets: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let mut output_paths: VecDeque<String> = matches
        .values_of("o")
        .unwrap_or(clap::Values::default())
        .map(str::to_string)
        .collect();
    if output_paths.len() > targets.len() {
        die!("ERROR: More output paths provided than input files");
    }
    for file in targets.iter() {
        let text = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => die!(e),
        };
        let instructions = match InstructionParser::parse_buffer(&text) {
            Ok(s) => s,
            Err(e) => die!(e),
        };
        let output = output_paths.pop_front().unwrap_or(gen_output_path(file));
        match write_instructions(&instructions, &output) {
            Ok(()) => {}
            Err(e) => die!(e),
        }
    }
}

fn gen_output_path(input_path: &str) -> String {
    let buf = PathBuf::from(input_path);
    let base = buf.as_path().file_stem().unwrap_or(buf.as_os_str());
    let mut out = PathBuf::from(base);
    out.push(".ch8");
    out.into_os_string().into_string().unwrap()
}

fn write_instructions(buf: &[Instruction], path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    let bytes: Vec<u8> = buf.iter().map(Instruction::as_bytes).fold(
        Vec::with_capacity(buf.len()),
        |mut vec, (b1, b2)| {
            vec.push(b1);
            vec.push(b2);
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
