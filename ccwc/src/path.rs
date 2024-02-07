use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::pattern::Pattern;
use anyhow::Context;

pub fn handle_path_provided(path: &PathBuf, pattern: Pattern) -> String {
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

fn is_path(pattern: &Option<String>) -> bool {
    return match pattern {
        Some(non_empty_pattern) => Path::new(non_empty_pattern).is_file(),
        None => false,
    };
}

fn build_path(pattern: &Option<String>) -> Option<PathBuf> {
    return match pattern {
        Some(ref pattern_str) => Some(PathBuf::from(pattern_str)),
        None => None,
    };
}

pub fn get_file_path(
    args_path: &Option<PathBuf>,
    args_pattern: &Option<String>,
) -> Option<PathBuf> {
    if is_path(args_pattern) {
        return build_path(args_pattern);
    }
    return args_path.to_owned();
}

pub fn get_pattern(args_pattern: &Option<String>) -> Pattern {
    if is_path(args_pattern) {
        return Pattern::new(None);
    }
    return Pattern::new(args_pattern.to_owned());
}
