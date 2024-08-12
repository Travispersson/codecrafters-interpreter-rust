use std::num::NonZeroUsize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,

    // literals
    String,
    Number,

    // ID
    Identifier,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    // Special type meaning end of file
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "EOF"),
            Self::LeftParen => write!(f, "LEFT_PAREN"),
            Self::RightParen => write!(f, "RIGHT_PAREN"),
            Self::LeftBrace => write!(f, "LEFT_BRACE"),
            Self::RightBrace => write!(f, "RIGHT_BRACE"),
            Self::Comma => write!(f, "COMMA"),
            Self::Dot => write!(f, "DOT"),
            Self::Minus => write!(f, "MINUS"),
            Self::Plus => write!(f, "PLUS"),
            Self::Semicolon => write!(f, "SEMICOLON"),
            Self::Star => write!(f, "STAR"),
            Self::Bang => write!(f, "BANG"),
            Self::BangEqual => write!(f, "BANG_EQUAL"),
            Self::Equal => write!(f, "EQUAL"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL"),
            Self::Less => write!(f, "LESS"),
            Self::LessEqual => write!(f, "LESS_EQUAL"),
            Self::Greater => write!(f, "GREATER"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL"),
            Self::Slash => write!(f, "SLASH"),
            Self::String => write!(f, "STRING"),
            Self::Number => write!(f, "NUMBER"),
            Self::Identifier => write!(f, "IDENTIFIER"),
            Self::And => write!(f, "AND"),
            Self::Class => write!(f, "AND"),
            Self::Else => write!(f, "AND"),
            Self::False => write!(f, "AND"),
            Self::For => write!(f, "AND"),
            Self::Fun => write!(f, "AND"),
            Self::If => write!(f, "AND"),
            Self::Nil => write!(f, "AND"),
            Self::Or => write!(f, "AND"),
            Self::Print => write!(f, "AND"),
            Self::Return => write!(f, "AND"),
            Self::Super => write!(f, "AND"),
            Self::This => write!(f, "AND"),
            Self::True => write!(f, "AND"),
            Self::Var => write!(f, "AND"),
            Self::While => write!(f, "AND"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    // Bool(..)
    None,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "null"),
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{:?}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: NonZeroUsize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Literal,
        line: NonZeroUsize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal,)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_print() {
        let tk = Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: Literal::None,
            line: NonZeroUsize::new(1).unwrap(),
        };
        let print = tk.to_string();

        assert_eq!(print, String::from("EOF  null"))
    }
}
