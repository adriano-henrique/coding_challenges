use std::{fs, path::PathBuf};

mod validator;
mod parser;

use clap::Parser;

use crate::{parser::tokenize, validator::validate};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    let file_path = PathBuf::from(args.path);

    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    let tokenized_contents = tokenize(contents);
    println!("{:?}", tokenized_contents);
    println!("{:?}", validate(tokenized_contents))
}
