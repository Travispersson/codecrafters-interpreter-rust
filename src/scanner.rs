use std::sync::OnceLock;
use std::{collections::HashMap, num::NonZeroUsize};

use crate::syntax::token::{Literal, Token, TokenType};

static RESERVED_KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

#[derive(Debug)]
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
            line: unsafe { NonZeroUsize::new_unchecked(1) },
            has_error: false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if let Some(c) = c {
            self.current += c.len_utf8();
        }

        c
    }

    fn advance_if_match(&mut self, to_match: char) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        match self.advance() {
            Some(c) => {
                if c == to_match {
                    Some(c)
                } else {
                    self.current -= c.len_utf8();
                    None
                }
            }
            _ => None,
        }
    }

    fn peek(&self) -> Option<char> {
        // https://doc.rust-lang.org/std/iter/struct.Peekable.html We could look into this for chars vec
        // because this is ugly af...
        // also this makes indexing O(n) ... not nice :S
        match self.source.char_indices().find(|(o, _)| *o == self.current) {
            None => None,
            Some((_, c)) => Some(c),
        }
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

    fn increase_line(&mut self) {
        unsafe { self.line = NonZeroUsize::new_unchecked(self.line.get() + 1) }
    }

    fn add_string(&mut self) {
        while self.peek() != Some('"') && !self.peek().is_none() && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.increase_line()
            }

            self.advance();
        }

        if self.is_at_end() {
            self.has_error = true;
            eprintln!("[line {}] Error: Unterminated string.", self.line.get(),);
            return;
        }

        // the closing "
        self.advance();

        let val = &self.source[self.start + '"'.len_utf8()..self.current - '"'.len_utf8()];
        self.add_token(TokenType::String, Literal::String(val.to_string()));
    }

    fn add_number(&mut self) {
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek() == Some('.') {
            self.current += '.'.len_utf8();
            if self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
                while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                    self.advance();
                }
            } else {
                self.current -= '.'.len_utf8();
            }
        }

        let lexeme = &self.source[self.start..self.current];
        self.add_token(
            TokenType::Number,
            Literal::Number(lexeme.parse().expect("should be a valid number")),
        );
    }

    fn add_identifier(&mut self) {
        while self
            .peek()
            .map_or(false, |c| c.is_alphanumeric() || c == '_')
        {
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];

        let x = RESERVED_KEYWORDS.get_or_init(|| {
            let mut rkw = HashMap::new();
            rkw.insert("and", TokenType::And);
            rkw.insert("class", TokenType::Class);
            rkw.insert("else", TokenType::Else);
            rkw.insert("false", TokenType::False);
            rkw.insert("for", TokenType::For);
            rkw.insert("fun", TokenType::Fun);
            rkw.insert("if", TokenType::If);
            rkw.insert("nil", TokenType::Nil);
            rkw.insert("or", TokenType::Or);
            rkw.insert("print", TokenType::Print);
            rkw.insert("return", TokenType::Return);
            rkw.insert("super", TokenType::Super);
            rkw.insert("this", TokenType::This);
            rkw.insert("true", TokenType::True);
            rkw.insert("var", TokenType::Var);
            rkw.insert("while", TokenType::While);
            rkw
        });

        match x.get(lexeme) {
            Some(tt) => self.add_token_without_literal(*tt),
            _ => self.add_token_without_literal(TokenType::Identifier),
        };
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        let Some(c) = c else {
            return;
        };

        match c {
            // single-character tokens
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

            // single-or-double character tokens
            '!' => match self.advance_if_match('=') {
                Some(_) => self.add_token_without_literal(TokenType::BangEqual),
                _ => self.add_token_without_literal(TokenType::Bang),
            },
            '=' => match self.advance_if_match('=') {
                Some(_) => self.add_token_without_literal(TokenType::EqualEqual),
                _ => self.add_token_without_literal(TokenType::Equal),
            },
            '>' => match self.advance_if_match('=') {
                Some(_) => self.add_token_without_literal(TokenType::GreaterEqual),
                _ => self.add_token_without_literal(TokenType::Greater),
            },
            '<' => match self.advance_if_match('=') {
                Some(_) => self.add_token_without_literal(TokenType::LessEqual),
                _ => self.add_token_without_literal(TokenType::Less),
            },
            '/' => match self.advance_if_match('/') {
                // We do not create a token for comments
                Some(_) => {
                    while self.peek() != Some('\n') && !self.peek().is_none() && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => self.add_token_without_literal(TokenType::Slash),
            },

            // ignore whitespace
            ' ' | '\r' | '\t' => {}

            // new lines
            '\n' => self.increase_line(),

            // string literals
            '"' => self.add_string(),

            // catch-all unsupported tokens
            _ => {
                if c.is_ascii_digit() {
                    self.add_number();
                } else if c.is_alphabetic() || c == '_' {
                    self.add_identifier();
                } else {
                    self.has_error = true;
                    eprintln!(
                        "[line {}] Error: Unexpected character: {}",
                        self.line.get(),
                        c
                    );
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&[Token], &[Token]> {
        while !self.is_at_end() {
            self.scan_token();
            self.start = self.current;
        }

        self.add_token_without_literal(TokenType::Eof);

        match self.has_error {
            false => Ok(&self.tokens),
            _ => Err(&self.tokens),
        }
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

        let tokens = scanner.scan_tokens().unwrap();
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

        let tokens = scanner.scan_tokens().unwrap();
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

        let tokens = scanner.scan_tokens().unwrap();
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

        let tokens = scanner.scan_tokens().unwrap();
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

    #[test]
    fn test_part_5() {
        let contents = ",.$(#";
        let mut scanner = Scanner::from(contents);

        let res = scanner.scan_tokens();
        let expected_tokens = [
            Token::new(
                TokenType::Comma,
                String::from(","),
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
                TokenType::LeftParen,
                String::from("("),
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
        match res {
            Err(tokens) => {
                for (i, token) in tokens.iter().enumerate() {
                    assert_eq!(*token, expected_tokens[i])
                }
            }
            _ => {
                res.expect_err("Test is incorrect, this should be an error");
            }
        }
    }

    #[test]
    fn test_assignment_and_equal_mix() {
        let contents = "=(==)";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::Equal,
                String::from("="),
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
                TokenType::EqualEqual,
                String::from("=="),
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
    fn test_ignore_comment() {
        let contents = "(//this is a comment";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::LeftParen,
                String::from("("),
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
    fn test_unicode() {
        let contents = "(///Unicode:£§᯽☺♣)";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::LeftParen,
                String::from("("),
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
    fn test_ignore_whitespace() {
        let contents = " /  \t \r";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::Slash,
                String::from("/"),
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
    fn test_string_literal() {
        let contents = "\"foo bar\"";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::String,
                String::from("\"foo bar\""),
                Literal::String("foo bar".to_string()),
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
    fn test_numbers() {
        let contents = "1234.1234";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::Number,
                String::from("1234.1234"),
                Literal::Number(1234.1234),
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
    fn test_identifiers() {
        let contents = "foo bar _hello";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::Identifier,
                String::from("foo"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Identifier,
                String::from("bar"),
                Literal::None,
                NonZeroUsize::new(1).unwrap(),
            ),
            Token::new(
                TokenType::Identifier,
                String::from("_hello"),
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
    fn test_reserved_keywords() {
        let contents = "and";
        let mut scanner = Scanner::from(contents);

        let tokens = scanner.scan_tokens().unwrap();
        let expected_tokens = [
            Token::new(
                TokenType::And,
                String::from("and"),
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
