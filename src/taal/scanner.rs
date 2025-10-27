use super::{
    SourceType, TaalError,
    token::{Token, TokenType},
};

/// trait to check if a u8 is alphabetic or underscore
trait MyAlpha {
    fn is_alpha(&self) -> bool;
}

impl MyAlpha for u8 {
    fn is_alpha(&self) -> bool {
        if *self == b'_' || self.is_ascii_alphabetic() {
            return true;
        }
        false
    }
}

#[derive(Debug, Default)]
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
            line: 1,
            ..Default::default()
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
    fn peek_is_end(&self, index: usize) -> bool {
        // we assume that every byte is character (so our language "taal" can exists in ASCII)
        self.current_in_lexeme + index >= self.source.len()
    }

    fn peek(&self, index: usize) -> u8 {
        if self.peek_is_end(index) {
            return b'\0';
        }
        self.source[self.current_in_lexeme + index]
    }

    fn add_token_with_literal<T>(&mut self, token_type: TokenType, text: T)
    where
        T: Into<SourceType>,
    {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start_of_lexeme..self.current_in_lexeme + 1].to_vec(),
            Some(text.into()),
            self.line,
        ));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start_of_lexeme..self.current_in_lexeme + 1].to_vec(),
            None,
            self.line,
        ));
    }

    fn match_and_add_token(
        &mut self,
        expected: u8,
        match_type: TokenType,
        mismatch_type: TokenType,
    ) {
        if self.peek(1) == expected {
            self.advance(); // consume matched char
            self.add_token(match_type);
        } else {
            self.add_token(mismatch_type);
        }
    }

    fn match_string(&mut self) -> Result<(), TaalError> {
        while self.peek(1) != b'"' && !self.peek_is_end(1) {
            if self.peek(1) == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.peek_is_end(1) {
            return Err(TaalError {
                message: "Unterminated string".to_string(),
                message_where: "".to_string(),
                line: self.line,
            });
        }

        // consume the closing "
        self.advance();

        let value = (&self.source[(self.start_of_lexeme + 1)..self.current_in_lexeme]).to_vec();
        self.add_token_with_literal(TokenType::String, value);
        Ok(())
    }

    fn match_number(&mut self) -> Result<(), TaalError> {
        // consume digits
        while self.peek(1).is_ascii_digit() {
            self.advance();
        }

        if self.peek(1) == b'.' && self.peek(2).is_ascii_digit() {
            // consume the .
            self.advance()
        }

        // consume digits
        while self.peek(1).is_ascii_digit() {
            self.advance();
        }

        let value = (&self.source[(self.start_of_lexeme)..self.current_in_lexeme + 1]).to_vec();
        // TODO in the text, they insert a real double here
        // for us, this conversion still has to happen later
        self.add_token_with_literal(TokenType::Number, value);
        Ok(())
    }

    /// identifies if the given source is a keyword
    /// if not, it is an identifier
    fn identify_keyword(&self, candidate: &[u8]) -> Result<TokenType, TaalError> {
        // using the from_utf8 function to convert &[u8] to &str to make my life easier
        // more appropriate way could be to match on &[u8] directly
        // the downside of using from_utf8 is that it can fail, so we have to handle that error
        if let Ok(s) = std::str::from_utf8(candidate) {
            return match s {
                "and" => Ok(TokenType::And),
                "class" => Ok(TokenType::Class),
                "else" => Ok(TokenType::Else),
                "false" => Ok(TokenType::False),
                "for" => Ok(TokenType::For),
                "fun" => Ok(TokenType::Fun),
                "if" => Ok(TokenType::If),
                "nil" => Ok(TokenType::Nil),
                "or" => Ok(TokenType::Or),
                "print" => Ok(TokenType::Print),
                "return" => Ok(TokenType::Return),
                "super" => Ok(TokenType::Super),
                "this" => Ok(TokenType::This),
                "true" => Ok(TokenType::True),
                "var" => Ok(TokenType::Var),
                "while" => Ok(TokenType::While),
                _ => Ok(TokenType::Identifier), // match identifier
            };
        }
        Err(TaalError {
            message: "Could not convert source keyword/identifier to Utf8".to_string(),
            message_where: "".to_string(),
            line: self.line,
        })
    }

    fn match_identifier(&mut self) -> Result<(), TaalError> {
        // consume alphabetics, _ or digits
        while self.peek(1).is_alpha() || self.peek(1).is_ascii_digit() {
            self.advance();
        }

        let value = &self.source[(self.start_of_lexeme)..self.current_in_lexeme + 1];
        match self.identify_keyword(&value) {
            Ok(token_type) => {
                self.add_token(token_type);
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    fn match_comment(&mut self) -> bool {
        // match single line comments
        if self.peek(1) == b'/' {
            self.advance();
            while (self.peek(1) != b'\n') && !self.peek_is_end(1) {
                self.advance();
            }
            return true;
        } else if self.peek(1) == b'*' {
            // match multiple line comments
            self.advance();
            // keep going, can be multiple lines
            while !self.peek_is_end(1) {
                if self.peek(1) == b'\n' {
                    self.line += 1;
                } else if self.peek(1) == b'*' && self.peek(2) == b'/' {
                    self.advance(); // go to  *
                    self.advance(); // go to /
                    return true;
                }
                self.advance();
            }
        }
        false
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
                if !self.match_comment() {
                    self.add_token(TokenType::Slash);
                }
            }
            b' ' | b'\r' | b'\t' => {
                // Ignore whitespace.
            }
            b'\n' => self.line += 1,
            b'"' => self.match_string()?, // match string literals
            c if c.is_ascii_digit() => self.match_number()?, // match numbers
            c if c.is_alpha() => self.match_identifier()?, // match identifiers if first is
            // alphabetic
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
