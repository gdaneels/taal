use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path;
use std::fmt::Display;

mod scanner;
mod token;

#[derive(Debug)]
enum TaalError {
    IoError(std::io::Error),
    ParseError(String),
}

impl From<std::io::Error> for TaalError {
    fn from (err: std::io::Error) -> Self {
        TaalError::IoError(err)
    }
}

impl From<String> for TaalError {
    fn from (err: String) -> Self {
        TaalError::ParseError(err)
    }
}

impl Display for TaalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaalError::IoError(e) => write!(f, "Taal IO error: {}", e),
            TaalError::ParseError(msg) => write!(f, "Taal parse error: {}", msg),
        }
    }
}

impl Error for TaalError {}

fn run_prompt() {
    // Placeholder for file execution logic
    println!("Running prompt...");
    run("".to_string());
}

fn run_file(path: path::PathBuf) -> Result<(), TaalError> {
    // Placeholder for file execution logic
    println!("Executing file: {:?}", path);
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);

    run("".to_string());

    Ok(())
}

fn run(source: String) {
    // Placeholder for file execution logic
    println!("Running...");
    let scanner = scanner::Scanner::new(source);
    scanner.scan_tokens();
}

pub fn taal(path: Option<path::PathBuf>) {
    println!("Running pret with path: {:?}", path);
    if let Some(p) = path {
        if let Err(e) = run_file(p) {
            eprintln!("{e}");
        }
    } else {
        run_prompt();
    }
}
