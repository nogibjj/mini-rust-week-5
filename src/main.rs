use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: file_renamer <directory> <search_pattern> <replacement>");
        std::process::exit(1);
    }

    let directory = &args[1];
    let search_pattern = &args[2];
    let replacement = &args[3];

    let re = Regex::new(search_pattern).unwrap();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
    }
}

