mod path;
mod pattern;

use anyhow::Result;
use clap::Parser;
use path::{handle_path_not_provided, handle_path_provided};
use pattern::Pattern;

#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,
    pattern: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let pattern = Pattern::new(args.pattern);

    let result = match args.path {
        Some(non_empty_path) => handle_path_provided(non_empty_path, pattern),
        None => handle_path_not_provided(),
    };

    println!("{}", result);

    Ok(())
}
