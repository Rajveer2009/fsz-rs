use colored::*;
use std::path::Path;
use std::{env, fs};

fn main() {
    // Specify the path to the file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to the file as a command-line argument.");
        return;
    }
    let file_path = &args[1];

    // Attempt to get metadata for the file
    let metadata = match fs::metadata(file_path) {
        Ok(metadata) => metadata,
        Err(_) => {
            eprintln!("Failed to get metadata for the file.");
            return;
        }
    };

    // Extract the file size from the metadata
    let file_size = metadata.len() as f64;

    // Determine the appropriate size metric
    let (file_size, size_metric) = if file_size < 1000.0 {
        // bytes
        (file_size, "bytes")
    } else if file_size < 1000.0 * 1000.0 {
        // kilobytes
        (file_size / 1000.0, "kilobytes")
    } else if file_size < 1000.0 * 1000.0 * 1000.0 {
        // megabytes
        (file_size / (1000.0 * 1000.0), "megabytes")
    } else {
        // gigabytes
        (file_size / (1000.0 * 1000.0 * 1000.0), "gigabytes")
    };

    // Round the file size

    // file size
    let rounded_file_size = ((file_size * 100.0).round() / 100.0).to_string();
    let formatted_file_size = rounded_file_size.bright_red();

    // size metric
    let formatted_size_metric = size_metric.bright_red();

    if let Some(file_name) = get_file_name(file_path) {
        println!(
            "{} -> {} {}",
            format!("\x1B[4m{}\x1B[0m", file_name).bright_yellow(),
            formatted_file_size,
            formatted_size_metric
        );
    } else {
        println!("Invalid file path.");
    }
}

fn get_file_name(file_path: &str) -> Option<&str> {
    // Convert the file path to a Path object
    let path = Path::new(file_path);

    // Get the file name component
    match path.file_name() {
        Some(file_name) => file_name.to_str(),
        None => None,
    }
}
