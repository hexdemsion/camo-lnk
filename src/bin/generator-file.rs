// cargo run --bin generator-file -- --target-path "C:\Users\IEUser\Desktop\dummyfile\" --use-extension ".docx,.pdf,.xlsx,.pptx" --file-amount 500 --range-size "1024-4096"
// cargo run --bin generator-file -- --target-path "C:\Users\IEUser\Desktop\dummyfile\" --use-extension ".txt,.json" --file-amount 500 --range-size "124-2096"

use std::fs;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use rand::Rng;
use rand::seq::SliceRandom;
use sha2::{Sha256, Digest};
use clap::{App, Arg};
use chrono::Local;
use rand::prelude::IndexedRandom;
use rand::distr::Alphanumeric;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("File Generator")
        .version("1.0")
        .author("Claude")
        .about("Generates dummy files with random content")
        .arg(Arg::with_name("target-path")
            .long("target-path")
            .value_name("PATH")
            .help("Target directory to place generated files")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("use-extension")
            .long("use-extension")
            .value_name("EXTENSIONS")
            .help("Comma-separated list of file extensions to use (include dots)")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("file-amount")
            .long("file-amount")
            .value_name("NUMBER")
            .help("Number of files to generate")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("range-size")
            .long("range-size")
            .value_name("RANGE")
            .help("Size range in KB (format: min-max)")
            .required(true)
            .takes_value(true))
        .get_matches();

    // Parse arguments
    let target_path = matches.value_of("target-path").unwrap();
    let extensions: Vec<&str> = matches.value_of("use-extension").unwrap()
        .split(',')
        .collect();
    let file_amount: usize = matches.value_of("file-amount").unwrap()
        .parse()?;
    
    let range_size = matches.value_of("range-size").unwrap();
    let range_parts: Vec<&str> = range_size.split('-').collect();
    if range_parts.len() != 2 {
        return Err("Range size must be in format 'min-max'".into());
    }
    
    let min_size: usize = range_parts[0].parse()?;
    let max_size: usize = range_parts[1].parse()?;
    
    if min_size > max_size {
        return Err("Minimum size cannot be greater than maximum size".into());
    }

    // Create directory if it doesn't exist
    let path = Path::new(target_path);
    if !path.exists() {
        fs::create_dir_all(path)?;
        println!("Created directory: {}", target_path);
    }

    let start_time = Local::now();
    println!("Starting file generation at: {}", start_time.format("%H:%M:%S"));
    println!("Generating {} files in {}", file_amount, target_path);
    println!("File size range: {}-{} KB", min_size, max_size);
    println!("Using extensions: {}", extensions.join(", "));

    let mut rng = rand::thread_rng();
    let mut files_created = 0;
    let mut errors = 0;

    for _ in 0..file_amount {
        // Generate random hash for filename
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        
        let mut hasher = Sha256::new();
        hasher.update(random_string);
        let hash = hasher.finalize();
        let filename = format!("{:x}", hash);
        
        // Choose random extension
        let extension = extensions.choose(&mut rng).unwrap();
        
        // Determine random file size within range
        let size_kb = rng.gen_range(min_size..=max_size);
        let size_bytes = size_kb * 1024;
        
        // Create full path
        let full_path = path.join(format!("{}{}", filename, extension));
        
        // Generate random content
        let mut file = match fs::File::create(&full_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error creating file {}: {}", full_path.display(), e);
                errors += 1;
                continue;
            }
        };
        
        // Write random data
        let mut bytes_written = 0;
        let buffer_size = 4096;
        
        while bytes_written < size_bytes {
            let chunk_size = std::cmp::min(buffer_size, size_bytes - bytes_written);
            let mut buffer = vec![0u8; chunk_size];
            rng.fill(&mut buffer[..]);
            
            match file.write_all(&buffer) {
                Ok(_) => bytes_written += chunk_size,
                Err(e) => {
                    println!("Error writing to file {}: {}", full_path.display(), e);
                    errors += 1;
                    break;
                }
            }
        }
        
        // Flush to ensure all data is written
        if let Err(e) = file.flush() {
            println!("Error flushing file {}: {}", full_path.display(), e);
            errors += 1;
            continue;
        }
        
        files_created += 1;
        
        // Progress update every 50 files
        if files_created % 50 == 0 {
            println!("Progress: {}/{} files created", files_created, file_amount);
        }
    }

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    
    println!("\nFile generation completed at: {}", end_time.format("%H:%M:%S"));
    println!("Time taken: {} seconds", duration.num_seconds());
    println!("Files created: {}", files_created);
    
    if errors > 0 {
        println!("Errors encountered: {}", errors);
    }

    Ok(())
}