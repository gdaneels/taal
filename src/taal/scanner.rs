use super::{
    SourceType, TaalError,
    token::{Token, TokenType},
};

pub struct Scanner {
    // We use the String type to enjoy the print functionality, but we assume that each character
    // fits nicely in 1 byte. That is, we assume the ASCII encoding to fit our entire taal
    // language. When traversing taal source code, we should be safe to convert the string it to bytes().
    source: SourceType,
    tokens: Vec<Token>,
    line: u32,
    start_of_lexeme: u32,   // index of line, of start character of lexeme
    current_in_lexeme: u32, // index of line, of current character in lexeme
}

impl Scanner {
    pub fn new(source: SourceType) -> Self {
        Self {
            source,
            tokens: vec![],
            line: 1,
            start_of_lexeme: 0,
            current_in_lexeme: 0,
        }
    }

    /// Advance the current character in the lexeme by one
    fn advance(&mut self) {
        self.current_in_lexeme += 1;
    }

    /// Returns true if the next character is the end of the source
    fn at_end(&self) -> bool {
        // we assume that every byte is character (so our language "taal" can exists in ASCII)
        self.current_in_lexeme + 1 >= (self.source.len() as u32)
    }

    fn peek_next(&self) -> u8 {
        if self.at_end() {
            return b'\0';
        }
        self.source[(self.current_in_lexeme + 1) as usize]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text =
            self.source[self.start_of_lexeme as usize..self.current_in_lexeme as usize].to_vec();
        self.tokens
            .push(Token::new(token_type, text, None, self.line));
    }

    fn match_and_add_token(
        &mut self,
        expected: u8,
        match_type: TokenType,
        mismatch_type: TokenType,
    ) {
        if self.peek_next() == expected {
            self.advance(); // consume matched char
            self.add_token(match_type);
        } else {
            self.add_token(mismatch_type);
        }
    }

    fn scan_token(&mut self) -> Result<(), TaalError> {
        let current_character = self.source[self.current_in_lexeme as usize];
        match current_character {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            b'!' => self.match_and_add_token(b'=', TokenType::BangEqual, TokenType::Bang),
            b'=' => self.match_and_add_token(b'=', TokenType::EqualEqual, TokenType::Equal),
            b'<' => self.match_and_add_token(b'=', TokenType::LessEqual, TokenType::Less),
            b'>' => self.match_and_add_token(b'=', TokenType::GreaterEqual, TokenType::Greater),
            b'/' => {
                if self.peek_next() == b'/' {
                    self.advance();
                    while (self.peek_next() != b'\n') && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            b' ' | b'\r' | b'\t' => {
                // Ignore whitespace.
            }
            b'\n' => self.line += 1,
            _ => {
                return Err(TaalError {
                    message: "Literal unknown".to_string(),
                    message_where: "".to_string(),
                    line: self.line,
                });
            }
        };

        // go to next character 
        self.advance();

        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<(), TaalError> {
        println!("Scanning tokens...");

        while !self.at_end() {
            self.start_of_lexeme = self.current_in_lexeme;
            self.scan_token()?;
        }

        // TODO parameters have to be corrected
        self.tokens
            .push(Token::new(TokenType::Eof, vec![], None, self.line));

        Ok(())
    }

    pub fn print_tokens(&self) {
        println!("Tokens: {:?}", self.tokens);
    }
}
