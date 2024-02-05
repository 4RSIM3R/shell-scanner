use regex::Regex;
use std::{env, fs::{self, OpenOptions}, time::Instant};
use walkdir::WalkDir;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage : {} path", args[0]);
        std::process::exit(1);
    }

    let clone = &mut args.to_owned();
    let pattern = &clone[1];
    let scan_path = &clone[2];

    let patterns_content = fs::read_to_string(pattern).expect("Could not read patterns file");
    let patterns: Vec<Regex> = patterns_content
        .lines()
        .map(|pattern| Regex::new(pattern.trim()).expect("Failed to compile regex"))
        .collect();

    let start = Instant::now();

    let mut matched_pattern = Vec::new();

    for entry in WalkDir::new(scan_path).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_string_lossy();
        let path = entry.path();

    
        for pattern in &patterns {
            if pattern.is_match(&file_name) {
                matched_pattern.push(path.to_owned());
                break;
            }
        }

    
    }

    let file_path = "matched_files.report";
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Failed to open report file");

    // Write the matched files to the report file, appending if the file already exists
    for path in &matched_pattern {
        writeln!(file, "{}", path.display()).expect("Failed to write to report file");
    }


    let duration = start.elapsed();
    println!("\nTime elapsed: {:?}", duration);
}
