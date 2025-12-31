use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Identifier(&'a str),
    OpenBrace,
    CloseBrace,
    Comma,
}

pub struct Lexer<'a> {
    src: &'a str,
    chars: Peekable<Chars<'a>>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.chars().peekable(),
            cursor: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();

        let start = self.cursor;
        let c = self.chars.next()?;
        self.cursor += c.len_utf8();

        match c {
            '{' => Some(Token::OpenBrace),
            '}' => Some(Token::CloseBrace),
            ',' => Some(Token::Comma),
            _ if c.is_alphabetic() || c == '_' => {
                Some(self.read_identifier(start))
            }
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.chars.next();
            self.cursor += c.len_utf8();
        }
    }

    fn read_identifier(&mut self, start: usize) -> Token<'a> {
        while let Some(&c) = self.chars.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.chars.next();
            self.cursor += c.len_utf8();
        }
        Token::Identifier(&self.src[start..self.cursor])
    }
}