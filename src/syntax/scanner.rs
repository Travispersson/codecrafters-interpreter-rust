use std::iter::Peekable;
use std::str::Chars;
use std::sync::OnceLock;
use std::{collections::HashMap, num::NonZeroUsize};

use crate::syntax::token::{Literal, Token, TokenType};

static RESERVED_KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    c_iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,   // start offset
    current: usize, // current offset
    line: NonZeroUsize,
    pub has_error: bool,
}

impl<'a> Scanner<'a> {
    fn new(buffer: &'a str) -> Self {
        Self {
            source: buffer,
            c_iter: buffer.chars().peekable(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: unsafe { NonZeroUsize::new_unchecked(1) },
            has_error: false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.c_iter.next().and_then(|v| {
            self.current += v.len_utf8();
            Some(v)
        })
    }

    fn advance_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        let Some(c) = self.peek() else {
            return None;
        };

        if !predicate(*c) {
            return None;
        }

        self.advance()
    }

    fn advance_while<F: Fn(char) -> bool>(&mut self, predicate: F) {
        while let Some(c) = self.peek() {
            if !predicate(*c) {
                break;
            }

            self.advance();
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.c_iter.peek()
    }

    fn peek_next(&mut self) -> Option<char> {
        // INVESTIGATE better way to do this?
        let mut iter = self.c_iter.clone();
        iter.next();
        // return next again instead of peek since peek returns a ref to the underlying value
        // this breaks borrowing rules since the valeus of the cloned iter does not live past this
        // function scope
        iter.next()
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::None)
    }

    fn lexeme(&self) -> &'a str {
        &self.source[self.start..self.current]
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let token = Token::new(token_type, self.lexeme().to_string(), literal, self.line);
        self.tokens.push(token);
    }

    fn increase_line(&mut self) {
        unsafe { self.line = NonZeroUsize::new_unchecked(self.line.get() + 1) }
    }

    fn add_string(&mut self) {
        while let Some(c) = self.peek() {
            if *c == '"' {
                break;
            }

            if *c == '\n' {
                self.increase_line();
            }

            self.advance();
        }

        if self.peek().is_none() {
            self.has_error = true;
            eprintln!("[line {}] Error: Unterminated string.", self.line.get(),);
            return;
        }

        self.advance();

        let s =
            self.lexeme()[self.start + '"'.len_utf8()..self.current - '"'.len_utf8()].to_string();
        self.add_token(TokenType::String, Literal::String(s));
    }

    fn add_number(&mut self) {
        self.advance_while(|c| c.is_ascii_digit());

        if self.peek() == Some(&'.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // consume '.'
            self.advance_while(|c| c.is_ascii_digit());
        }

        let num = self
            .lexeme()
            .parse::<f64>()
            .expect("Should be a valid number");
        self.add_token(TokenType::Number, Literal::Number(num));
    }

    fn add_identifier(&mut self, c: char) {
        self.advance_while(|ch| ch.is_alphanumeric() || ch == '_');
        let lexeme = self.lexeme();

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
            self.add_token_without_literal(TokenType::Eof);
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
            '!' => match self.advance_if(|c| c == '=') {
                Some(_) => self.add_token_without_literal(TokenType::BangEqual),
                _ => self.add_token_without_literal(TokenType::Bang),
            },
            '=' => match self.advance_if(|c| c == '=') {
                Some(_) => self.add_token_without_literal(TokenType::EqualEqual),
                _ => self.add_token_without_literal(TokenType::Equal),
            },
            '>' => match self.advance_if(|c| c == '=') {
                Some(_) => self.add_token_without_literal(TokenType::GreaterEqual),
                _ => self.add_token_without_literal(TokenType::Greater),
            },
            '<' => match self.advance_if(|c| c == '=') {
                Some(_) => self.add_token_without_literal(TokenType::LessEqual),
                _ => self.add_token_without_literal(TokenType::Less),
            },
            '/' => match self.advance_if(|c| c == '/') {
                // We do not create a token for comments
                Some(_) => self.advance_while(|c| c != '\n'),
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
                    self.add_identifier(c);
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
        while !self.c_iter.peek().is_none() {
            self.start = self.current;
            self.scan_token();
        }

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
    fn test_empty_source() {
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
    fn test_parantheses() {
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
    fn test_braces() {
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
    fn test_rest_of_single_character_tokens() {
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
    fn test_errors() {
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
