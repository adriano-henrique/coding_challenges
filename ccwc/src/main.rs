use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    pattern: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", args.path.display()))?;

    match args.pattern {
        Some(non_empty_pattern) => handle_command_provided(non_empty_pattern, content, args.path),
        None => handle_command_not_provided(content, args.path),
    }

    Ok(())
}

fn handle_command_provided(pattern: String, content: String, path: std::path::PathBuf) {
    match pattern.as_str() {
        "-c" => {
            let byte_count = content.len();
            println!("{} {:?}", byte_count, &path);
        }
        "-l" => {
            let line_count = content.lines().count();
            println!("{} {:?}", line_count, &path);
        }
        "-w" => {
            let word_count = content.split_whitespace().count();
            println!("{} {:?}", word_count, &path);
        }
        "" => {
            println!("Test");
        }
        _ => println!("Command not found. Use -help to understand more"),
    }
}

fn handle_command_not_provided(content: String, path: std::path::PathBuf) {
    let byte_count = content.len();
    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();

    println!("{} {} {} {:?}", byte_count, line_count, word_count, &path);
}
