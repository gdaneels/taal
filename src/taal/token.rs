use super::SourceType;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String,
    Number,

    // keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: SourceType,
    literal: Option<SourceType>, // what should type of literal be?
    line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            Some(literal) => write!(f, "Token [token_type: {:?}, lexeme: {}, literal: {}, line: {}]", self.token_type, self.lexeme, literal, self.line),
            None => write!(f, "Token [token_type: {:?}, lexeme: {}, literal: [empty], line: {}]", self.token_type, self.lexeme, self.line),
        }
    }
}

impl Token {
    pub fn new<T>(token_type: TokenType, lexeme: T, literal: Option<SourceType>, line: usize) -> Self
    where T: Into<SourceType>,{
        Self {
            token_type,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }
}
