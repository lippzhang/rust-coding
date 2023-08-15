
use std::str::Chars;

struct Lexer<'a> {
    /// Source Text
    source: &'a str,

    /// The remaining characters
    chars: Chars<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars()
        }
    }
    fn read_next_kind(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            match c {
              '+' => return Kind::Plus,
              _ => {}
            }
        }
        Kind::Eof
    }

    fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token { kind, start, end }
    }

    /// Get the length offset from the source text, in UTF-8 bytes
    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof,
    Plus,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
       
    }
}
