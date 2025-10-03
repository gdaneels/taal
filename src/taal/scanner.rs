use super::{
    TaalError,
    token::{Token, TokenType},
};

pub struct Scanner {
    // We use the String type to enjoy the print functionality, but we assume that each character
    // fits nicely in 1 byte. That is, we assume the ASCII encoding to fit our entire taal
    // language. When traversing taal source code, we should be safe to convert the string it to bytes().
    source: String,
    tokens: Vec<Token>,
    line: u32,
    start_of_lexeme: u32,   // index of line, of start character of lexeme
    current_in_lexeme: u32, // index of line, of current character in lexeme
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            line: 1,
            start_of_lexeme: 0,
            current_in_lexeme: 0,
        }
    }

    fn at_end(&self) -> bool {
        // we assume that every byte is character (so our language "taal" can exists in ASCII)
        self.current_in_lexeme >= (self.source.bytes().count() as u32)
    }

    fn get_token_type(&self, character: u8) -> Result<TokenType, TaalError> {
        let token_type = match character {
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b'{' => TokenType::LeftBrace,
            b'}' => TokenType::RightBrace,
            b',' => TokenType::Comma,
            b'.' => TokenType::Dot,
            b'-' => TokenType::Minus,
            b'+' => TokenType::Plus,
            b';' => TokenType::Semicolon,
            b'*' => TokenType::Star,
            _ => {
                return Err(TaalError {
                    message: "Literal unknown".to_string(),
                    message_where: "".to_string(),
                    line: self.line,
                });
            }
        };

        Ok(token_type)
    }

    fn scan_token(&mut self) -> Result<(), TaalError> {
        if let Some(character) = self.source.bytes().nth(self.current_in_lexeme as usize) {
            let token_type = self.get_token_type(character)?;
            let text = "TODO".to_string(); // maybe we can save the lexeme as a vec<u8> whenever we

            self.tokens
                .push(Token::new(token_type, text, None, self.line));
        }

        // advance to next character
        self.current_in_lexeme += 1;
        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<(), TaalError> {
        println!("Scanning tokens from source of length.");

        while !self.at_end() {
            self.start_of_lexeme = self.current_in_lexeme;
            self.scan_token()?;
        }

        // TODO parameters have to be corrected
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, 0));

        Ok(())
    }
}
