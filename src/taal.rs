use anyhow::{Context, Result};
use std::io::{BufRead, Read};
use std::path;

mod scanner;
mod token;

#[derive(Debug)]
pub struct TaalError {
    message: String,
    message_where: String,
    line: usize,
}

impl std::fmt::Display for TaalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TaalError [line {}] {}: {}",
            self.line, self.message_where, self.message
        )
    }
}

impl std::error::Error for TaalError {}

#[derive(Debug)]
struct SourceType(Vec<u8>);

impl From<Vec<u8>> for SourceType {
    fn from(value: Vec<u8>) -> Self {
        SourceType(value)
    }
}

impl std::ops::Deref for SourceType {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn run<T>(source: T) -> Result<(), TaalError>
where
    T: Into<SourceType>,
{
    // Placeholder for file execution logic
    println!("Running...");
    let mut scanner = scanner::Scanner::new(source.into());
    scanner.scan_tokens()?;
    scanner.print_tokens();
    Ok(())
}

fn run_prompt_mode() -> Result<(), anyhow::Error> {
    println!("------------------------------------------");
    println!("PROMPT MODE (Press Ctrl+D to exit)");
    println!("------------------------------------------");

    for line in std::io::stdin().lock().lines() {
        let line = line.context("Failed to read line from stdin")?;
        run(line.into_bytes())?;
    }

    Ok(())
}

fn run_file_mode(path: path::PathBuf) -> Result<(), anyhow::Error> {
    println!("------------------------------------------");
    println!("FILE MODE ({:?})", path);
    println!("------------------------------------------");

    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(contents.into_bytes())?;

    Ok(())
}

fn run_taal(path: Option<path::PathBuf>) -> Result<(), anyhow::Error> {
    match path {
        Some(p) => run_file_mode(p),
        None => run_prompt_mode(),
    }
}

pub fn taal(path: Option<path::PathBuf>) {
    println!("Running taal with path: {:?}", path);
    if let Err(e) = run_taal(path) {
        eprintln!("Error: {:?}", e);
    }
}
