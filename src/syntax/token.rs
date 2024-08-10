use std::fmt::write;

#[derive(Debug)]
enum TokenType {
    // single character tokens

    // one or two character tokens

    // literals

    // Keywords
    Eof,
}

#[derive(Debug)]
enum Literal {
    // String(..)
    // Number(..)
    // Bool(..)
    None,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?}",
            self.token_type, self.lexeme, self.literal,
        )
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
            line: 0,
        };
        let print = tk.to_string();

        assert_eq!(print, String::from("Eof \"\" None"))
    }
}
