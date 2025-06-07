use std::{iter::Peekable, str::Chars};

use crate::token::Token;

pub struct Lexer<'src> {
    chars: Peekable<Chars<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(code: &'src str) -> Self {
        let chars = code.chars().peekable();

        Self { chars }
    }

    #[inline]
    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.accept_next_token().ok()
    }

    fn accept_next_token(&mut self) -> Result<Token, ()> {
        let ret = match self.chars.peek().ok_or(())? {
            ' ' | '\n' | '\r' | '\t' => self.ignore_white_and_accept_next(),
            '0'..'9' => self.accept_int(),
            '+' => self.accept_single(Token::Plus),
            '(' => self.accept_single(Token::LeftParen),
            ')' => self.accept_single(Token::RightParen),
            _ => unreachable!(),
        };

        ret
    }

    // Utils

    #[inline]
    fn accept_single(&mut self, ty: Token) -> Result<Token, ()> {
        self.next_char();

        Ok(ty)
    }

    // Impls

    fn ignore_white_and_accept_next(&mut self) -> Result<Token, ()> {
        while self.chars.peek().is_some_and(|c| c.is_ascii_whitespace()) {
            self.next_char();
        }

        self.accept_next_token()
    }

    fn accept_int(&mut self) -> Result<Token, ()> {
        let mut buf = String::new();
        while self.chars.peek().is_some_and(|c| c.is_numeric()) {
            buf.push(self.next_char().unwrap());
        }

        Ok(Token::Int(buf.parse().map_err(|_| ())?))
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_num() {
        let mut lexer = Lexer::new("1 23 4 567 0");

        assert_eq!(Some(Token::Int(1)), lexer.next_token());
        assert_eq!(Some(Token::Int(23)), lexer.next_token());
        assert_eq!(Some(Token::Int(4)), lexer.next_token());
        assert_eq!(Some(Token::Int(567)), lexer.next_token());
        assert_eq!(Some(Token::Int(0)), lexer.next_token());
        assert_eq!(None, lexer.next_token());
    }

    #[test]
    fn unnested_additional_expr() {
        let mut lexer = Lexer::new("+ 1 2");

        assert_eq!(Some(Token::Plus), lexer.next_token());
        assert_eq!(Some(Token::Int(1)), lexer.next_token());
        assert_eq!(Some(Token::Int(2)), lexer.next_token());
        assert_eq!(None, lexer.next_token());
    }

    #[test]
    fn whitespace() {
        let mut lexer = Lexer::new("     \r\r\n\n\t  \n\n\n\t    ");

        assert_eq!(None, lexer.next_token());
    }

    #[test]
    fn parened_expr() {
        let mut lexer = Lexer::new("(1)");

        assert_eq!(Some(Token::LeftParen), lexer.next_token());
        assert_eq!(Some(Token::Int(1)), lexer.next_token());
        assert_eq!(Some(Token::RightParen), lexer.next_token());
        assert_eq!(None, lexer.next_char());
    }
}
