use std::num::NonZeroUsize;

use crate::syntax::token::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: NonZeroUsize,
}

impl<'a> Scanner<'a> {
    fn new(contents: &'a str) -> Self {
        Self {
            source: contents,
            tokens: vec![],
            start: 0,
            current: 0,
            line: NonZeroUsize::new(1).unwrap(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current = self.current + 1;
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

    fn scan_token(&self) {
        unimplemented!()
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token_without_literal(TokenType::Eof);
        return &self.tokens;
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
    fn impl_empty_source() {
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
}
