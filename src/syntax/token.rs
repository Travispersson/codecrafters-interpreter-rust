use std::num::NonZeroUsize;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,

    // one or two character tokens

    // literals

    // Keywords
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Eof => write!(f, "EOF"),
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    // String(..)
    // Number(..)
    // Bool(..)
    None,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::None => write!(f, "null"),
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
