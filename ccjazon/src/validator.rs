use std::{env, fs, path::PathBuf};

pub fn validate_json(path: &String) -> bool {
    let file_path = build_file_path(path);
    let test = fs::read_to_string(file_path).unwrap();
    println!("{:?}", test);
    return false;
}

fn build_file_path(path: &String) -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not get home directory path");
    let lib_dir = PathBuf::from(home_dir).join("dev/coding_challenges/ccjazon");
    let analyzed_file_path = PathBuf::from(lib_dir).join(path);
    println!("Analyzed directory: {}", analyzed_file_path.display());
    return analyzed_file_path;
}
