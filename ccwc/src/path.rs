use std::io::{self, Read};
use std::path::PathBuf;

use crate::pattern::Pattern;
use anyhow::Context;

pub fn handle_path_provided(path: PathBuf, pattern: Pattern) -> String {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{:?}`", path.display()))
        .unwrap();

    return pattern.get_pattern_type().to_string(&content)
        + &" ".to_string()
        + &path.display().to_string();
}

pub fn handle_path_not_provided(pattern: Pattern) -> String {
    let mut input_buffer = String::new();

    io::stdin()
        .read_to_string(&mut input_buffer)
        .with_context(|| format!("could not read input"))
        .unwrap();

    return pattern.get_pattern_type().to_string(&input_buffer);
}
