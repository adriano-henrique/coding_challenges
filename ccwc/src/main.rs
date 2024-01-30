mod pattern;

use anyhow::{Context, Result};
use clap::Parser;
use pattern::CountType;

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
            println!("{} {:?}", CountType::ByteCount.get_string(&content), &path);
        }
        "-l" => {
            println!("{} {:?}", CountType::LineCount.get_string(&content), &path);
        }
        "-w" => {
            println!("{} {:?}", CountType::WordCount.get_string(&content), &path);
        }
        "" => {
            println!("Test");
        }
        _ => println!("Command not found. Use -help to understand more"),
    }
}

fn handle_command_not_provided(content: String, path: std::path::PathBuf) {
    println!(
        "{} {} {} {:?}",
        CountType::ByteCount.get_string(&content),
        CountType::LineCount.get_string(&content),
        CountType::WordCount.get_string(&content),
        &path
    );
}
