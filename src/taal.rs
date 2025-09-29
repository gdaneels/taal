use anyhow::{Context, Result};
use std::io::Read;
use std::path;

mod scanner;
mod token;

fn run(source: String) {
    // Placeholder for file execution logic
    println!("Running...");
    let scanner = scanner::Scanner::new(source);
    scanner.scan_tokens();
}

fn run_prompt() -> Result<(), anyhow::Error> {
    // Placeholder for file execution logic
    println!("Running prompt...");
    let mut contents = String::new();
    std::io::stdin()
        .read_line(&mut contents)
        .context("Failed to read line from stdin")?;
    run("".to_string());
    Ok(())
}

fn run_file(path: path::PathBuf) -> Result<(), anyhow::Error> {
    // Placeholder for file execution logic
    println!("Executing file: {:?}", path);
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);

    run("".to_string());

    Ok(())
}

fn run_taal(path: Option<path::PathBuf>) -> Result<(), anyhow::Error> {
    match path {
        Some(p) => run_file(p),
        None => run_prompt(),
    }
}

pub fn taal(path: Option<path::PathBuf>) {
    println!("Running taal with path: {:?}", path);
    if let Err(e) = run_taal(path) {
        eprintln!("Error: {:?}", e);
    }
}
