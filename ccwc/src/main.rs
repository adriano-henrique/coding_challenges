mod path;
mod pattern;

use anyhow::Result;
use clap::Parser;
use path::{handle_path_not_provided, handle_path_provided};
use pattern::Pattern;

#[derive(Parser)]
struct Cli {
    pattern: Option<String>,
    path: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let pattern = Pattern::new(args.pattern);

    let result = match args.path {
        Some(non_empty_path) => handle_path_provided(non_empty_path, pattern),
        None => handle_path_not_provided(pattern),
    };

    println!("{}", result);

    Ok(())
}
