pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    // TODO Vec<String>?
    pub fn scan_tokens(&self) {
        println!("Scanning tokens from source of length.");
    }
}
