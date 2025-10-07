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
    line: usize,
    start_of_lexeme: usize,   // index of line, of start character of lexeme
    current_in_lexeme: usize, // index of line, of current character in lexeme
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

    /// Returns true if the current character is the end of the source
    fn at_end(&self) -> bool {
        // we assume that every byte is character (so our language "taal" can exists in ASCII)
        self.current_in_lexeme >= self.source.len()
    }

    /// Returns true if the next character is the end of the source
    fn peek_is_end(&self) -> bool {
        // we assume that every byte is character (so our language "taal" can exists in ASCII)
        self.current_in_lexeme + 1 >= self.source.len()
    }

    fn peek_next(&self) -> u8 {
        if self.peek_is_end() {
            return b'\0';
        }
        self.source[self.current_in_lexeme + 1]
    }

    fn add_token_with_literal<T>(&mut self, token_type: TokenType, text: T)
    where
        T: Into<SourceType>,
    {
        self.tokens
            .push(Token::new(token_type, vec![], Some(text.into()), self.line));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens
            .push(Token::new(token_type, vec![], None, self.line));
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

    fn match_string(&mut self) -> Result<(), TaalError> {
        while self.peek_next() != b'"' && !self.peek_is_end() {
            if self.peek_next() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.peek_is_end() {
            return Err(TaalError {
                message: "Unterminated string".to_string(),
                message_where: "".to_string(),
                line: self.line,
            });
        }

        // consume the closing "
        self.advance();

        let value = (&self.source[self.start_of_lexeme..(self.current_in_lexeme - 1)]).to_vec();
        self.add_token_with_literal(TokenType::String, value);
        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), TaalError> {
        let current_character = self.source[self.current_in_lexeme];
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
                    while (self.peek_next() != b'\n') && !self.peek_is_end() {
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
            b'"' => self.match_string()?,
            _ => {
                return Err(TaalError {
                    message: "Literal unknown".to_string(),
                    message_where: "".to_string(),
                    line: self.line,
                });
            }
        };

        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<(), TaalError> {
        println!("Scanning tokens...");

        while !self.at_end() {
            self.start_of_lexeme = self.current_in_lexeme;
            self.scan_token()?;

            // go to first character of next lexeme/token
            self.advance();
        }

        // TODO parameters have to be corrected
        self.tokens
            .push(Token::new(TokenType::Eof, vec![], None, self.line));

        Ok(())
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }
}
