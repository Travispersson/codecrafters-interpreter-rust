pub struct Scanner<'a> {
    contents: &'a str,
}

impl<'a> Scanner<'a> {
    fn new(contents: &'a str) -> Self {
        Self { contents }
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

        assert_eq!(scanner.contents, contents)
    }
}
