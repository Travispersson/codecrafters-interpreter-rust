use std::num::NonZeroUsize;

use crate::syntax::token::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: NonZeroUsize,
    pub has_error: bool,
}

impl<'a> Scanner<'a> {
    fn new(contents: &'a str) -> Self {
        Self {
            source: contents,
            tokens: vec![],
            start: 0,
            current: 0,
            line: NonZeroUsize::new(1).unwrap(),
            has_error: false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        if c.is_some() {
            self.current += 1;
        }

        c
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) {
        let literal = Literal::None;
        self.add_token(token_type, literal)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            lexeme.to_string(),
            literal,
            self.line,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        let Some(c) = c else {
            return;
        };

        match c {
            '(' => self.add_token_without_literal(TokenType::LeftParen),
            ')' => self.add_token_without_literal(TokenType::RightParen),
            '{' => self.add_token_without_literal(TokenType::LeftBrace),
            '}' => self.add_token_without_literal(TokenType::RightBrace),
            '.' => self.add_token_without_literal(TokenType::Dot),
            '*' => self.add_token_without_literal(TokenType::Star),
            '+' => self.add_token_without_literal(TokenType::Plus),
            '-' => self.add_token_without_literal(TokenType::Minus),
            ';' => self.add_token_without_literal(TokenType::Semicolon),
            ',' => self.add_token_without_literal(TokenType::Comma),
            _ => {
                self.has_error = true;
                eprintln!("{}: Unexpected character {}", self.line.get(), c);
            }
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.scan_token();
            self.start = self.current;
        }

        self.add_token_without_literal(TokenType::Eof);

        &self.tokens
    }
}

impl<'a> From<&'a str> for Scanner<'a> {
    fn from(value: &'a str) -> Self {
        Scanner::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_from_str_ref() {
        let contents = "test test";
        let scanner = Scanner::from(contents);

        assert_eq!(scanner.source, contents);
        assert!(scanner.tokens.len() == 0);
        assert_eq!(scanner.start, 0);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.line.get(), 1);
    }

    #[test]
    fn test_part_1() {
        let contents = "";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens();
        let expected_tokens = [Token::new(
            TokenType::Eof,
            String::from(""),
            Literal::None,
            NonZeroUsize::new(1).unwrap(),
        )];
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i])
        }
    }

    #[test]
    fn test_part_2() {
        let contents = "(()";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens();
        let expected_tokens = [
            Token::new(
                TokenType::LeftParen,
                String::from("("),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::LeftParen,
                String::from("("),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::RightParen,
                String::from(")"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Eof,
                String::from(""),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
        ];
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i])
        }
    }

    #[test]
    fn test_part_3() {
        let contents = "{{}}";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens();
        let expected_tokens = [
            Token::new(
                TokenType::LeftBrace,
                String::from("{"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::LeftBrace,
                String::from("{"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::RightBrace,
                String::from("}"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::RightBrace,
                String::from("}"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Eof,
                String::from(""),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
        ];
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i])
        }
    }

    #[test]
    fn test_part_4() {
        let contents = "({*.,+*})";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens();
        let expected_tokens = [
            Token::new(
                TokenType::LeftParen,
                String::from("("),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::LeftBrace,
                String::from("{"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Star,
                String::from("*"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Dot,
                String::from("."),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Comma,
                String::from(","),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Plus,
                String::from("+"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Star,
                String::from("*"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::RightBrace,
                String::from("}"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::RightParen,
                String::from(")"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Eof,
                String::from(""),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
        ];
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, expected_tokens[i])
        }
    }
}
