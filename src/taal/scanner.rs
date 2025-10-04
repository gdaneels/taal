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

    fn at_end(&self) -> bool {
        // we assume that every byte is character (so our language "taal" can exists in ASCII)
        self.current_in_lexeme >= (self.source.len() as u32)
    }

    fn is_expected(&self, expected: u8) -> bool {
        if self.at_end() {
            return false;
        }
        self.source[self.current_in_lexeme as usize] == expected
    }

    /// If the next character is `expected`, consume it (advance to next char) and return `first_type`, otherwise return
    fn match_next(
        &mut self,
        expected: u8,
        first_type: TokenType,
        second_type: TokenType,
    ) -> TokenType {
        if self.is_expected(expected) {
            self.current_in_lexeme += 1;
            return first_type;
        }
        second_type
    }

    fn get_token_type(&mut self) -> Result<TokenType, TaalError> {
        let current_character = self.source[self.current_in_lexeme as usize];
        // advance to next character, as we are consuming this one
        self.current_in_lexeme += 1;
        let token_type = match current_character {
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
            b'!' => self.match_next(b'=', TokenType::BangEqual, TokenType::Bang),
            b'=' => self.match_next(b'=', TokenType::EqualEqual, TokenType::Equal),
            b'<' => self.match_next(b'=', TokenType::LessEqual, TokenType::Less),
            b'>' => self.match_next(b'=', TokenType::GreaterEqual, TokenType::Greater),
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
        let token_type = self.get_token_type()?;
        // exclude the current character, as the get_token_type() function should have advanced until the next charactor of a new lexeme
        let text =
            self.source[self.start_of_lexeme as usize..self.current_in_lexeme as usize].to_vec();
        self.tokens
            .push(Token::new(token_type, text, None, self.line));
        println!("Tokens: {:?}", self.tokens);

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
            .push(Token::new(TokenType::Eof, vec![], None, 0));

        Ok(())
    }
}
