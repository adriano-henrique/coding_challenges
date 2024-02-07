mod path;
mod pattern;

extern crate anyhow;
extern crate clap;

use anyhow::Result;
use clap::Parser;
use path::{get_file_path, get_pattern, handle_path_not_provided, handle_path_provided};

#[derive(Parser)]
struct Cli {
    pattern: Option<String>,
    path: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file_path = get_file_path(&args.path, &args.pattern);
    let pattern = get_pattern(&args.pattern);

    let result = match file_path {
        Some(non_empty_path) => handle_path_provided(&non_empty_path, pattern),
        None => handle_path_not_provided(pattern),
    };

    println!("{}", result);

    Ok(())
}
