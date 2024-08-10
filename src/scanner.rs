use std::num::NonZeroUsize;

use crate::syntax::token::Token;

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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&self) {
        unimplemented!()
    }

    fn scan_tokens() {
        unimplemented!()
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
}
