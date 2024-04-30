use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use base64::prelude::*;
fn main() {
    // Owner and GitHub URL
    let owner = "Aly Ahmed Aly";
    let github_url = "https://github.com/King-Masr/Poius";

    // Parsing command-line arguments
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let path = args.get(2).map(|s| s.as_str()).unwrap_or("");

    // Default command for the CLI
    if command.is_empty() {
        println!("Welcome to poius CLI by {}.", owner);
        println!("Usage: poius <command> <path>");
        println!("Commands:");
        println!("  encode <file/dir>: Encode a file or directory");
        println!("  decode <file/dir>: Decode a file or directory");
        println!("  help: Display this help message");
        println!("For more information, visit {}", github_url);
        return;
    }

    // Handling encode, decode, and help commands
    match command {
        "encode" => {
            if path.is_empty() {
                println!("File or directory path not provided.");
                return;
            }
            if let Err(err) = encode(path) {
                println!("{}", err);
            }
        }
        "decode" => {
            if path.is_empty() {
                println!("File or directory path not provided.");
                return;
            }
            if let Err(err) = decode(path) {
                println!("{}", err);
            }
        }
        "help" => {
            println!("Usage: poius <command> <path>");
            println!("Commands:");
            println!("  encode <file/dir>: Encode a file or directory");
            println!("  decode <file/dir>: Decode a file or directory");
            println!("  help: Display this help message");
            println!("For more information, visit {}", github_url);
        }
        _ => println!("Invalid command. Use 'encode', 'decode', or 'help'."),
    }
}

// Encoding and decoding functions
fn encode(path: &str) -> io::Result<()> {
    if Path::new(path).is_file() {
        encode_file(path)?;
    } else if Path::new(path).is_dir() {
        encode_directory(path)?;
    } else {
        println!("File or directory not found.");
    }
    Ok(())
}

fn decode(path: &str) -> io::Result<()> {
    if Path::new(path).is_file() {
        if let Err(err) = decode_file(path) {
            // Convert the base64::DecodeError to std::io::Error if needed
            let io_err = io::Error::new(io::ErrorKind::Other, format!("Decode error: {}", err));
            return Err(io_err);
        }
    } else if Path::new(path).is_dir() {
        if let Err(err) = decode_directory(path) {
            // Convert the base64::DecodeError to std::io::Error if needed
            let io_err = io::Error::new(io::ErrorKind::Other, format!("Decode error: {}", err));
            return Err(io_err);
        }
    } else {
        println!("File or directory not found.");
    }
    Ok(())
}

// Function to encode a file
fn encode_file(file: &str) -> io::Result<()> {
    let mut content = Vec::new();
    File::open(file)?.read_to_end(&mut content)?;

    // Encode the content
    let encoded_content = BASE64_STANDARD.encode(&content);

    // Create a new file name for the encoded file
    let encoded_file = format!("{}.encoded", file);
    let encoded_file_str = encoded_file.clone();

    // Create the encoded file and write the encoded content
    File::create(&encoded_file)?.write_all(encoded_content.as_bytes())?;

    // Remove the original file
    fs::remove_file(file)?;

    println!("File encoded successfully. Encoded file: {}", encoded_file_str);
    Ok(())
}


// Function to encode a directory
fn encode_directory(dir: &str) -> io::Result<()> {
    let files = get_files_in_directory(dir)?;
    for file in files {
        encode_file(file.to_str().unwrap())?;
    }
    println!("Directory encoded successfully.");
    Ok(())
}

// Function to decode a file
fn decode_file(file: &str) -> io::Result<()> {
    let mut content = Vec::new();
    File::open(file)?.read_to_end(&mut content)?;

    // Attempt to decode the content
    let decoded_content = match BASE64_STANDARD.decode(&content) {
        Ok(data) => data,
        Err(err) => {
            // Convert the base64::DecodeError to std::io::Error if needed
            let io_err = io::Error::new(io::ErrorKind::Other, format!("Decode error: {}", err));
            return Err(io_err);
        }
    };

    // Create a new PathBuf for the decoded file
    let decoded_file = Path::new(file).with_extension("").to_path_buf(); // Remove .encoded extension
    let decoded_file_str = decoded_file.to_str().unwrap().to_owned(); // Convert PathBuf to String

    // Create the decoded file and write the decoded content
    File::create(&decoded_file)?.write_all(&decoded_content)?;

    // Remove the encoded file
    fs::remove_file(file)?;

    println!("File decoded successfully. Decoded file: {}", decoded_file_str);
    Ok(())
}


// Function to decode a directory
fn decode_directory(dir: &str) -> io::Result<()> {
    let files = get_files_in_directory(dir)?;
    for file in files {
        decode_file(file.to_str().unwrap())?;
    }
    println!("Directory decoded successfully.");
    Ok(())
}

// Helper function to get all files in a directory
fn get_files_in_directory(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}
