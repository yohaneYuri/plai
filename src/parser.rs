use crate::{ast::*, lexer::Lexer, token::Token};

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    lookahead: Option<Token>,
}

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedEof,
    UnexpectedToken(Token),
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
        let mut lexer = lexer;
        let lookahead = lexer.next_token();

        Self { lexer, lookahead }
    }

    #[inline]
    fn lookahead_ref(&self) -> Result<&Token, SyntaxError> {
        Ok(self.lookahead.as_ref().ok_or(SyntaxError::UnexpectedEof)?)
    }

    #[inline]
    fn match_token(&self, token: Token) -> Result<bool, SyntaxError> {
        Ok(self.lookahead_ref()?.eq(&token))
    }

    fn match_int(&self) -> Result<bool, SyntaxError> {
        Ok(if let Token::Int(_) = self.lookahead_ref()? {
            true
        } else {
            false
        })
    }

    #[inline]
    fn advance(&mut self) {
        self.lookahead = self.lexer.next_token();
    }

    #[inline]
    fn consume(&mut self, token: Token) -> Result<(), SyntaxError> {
        match self.lookahead.take() {
            None => Err(SyntaxError::UnexpectedEof),
            Some(lookahead) if token == lookahead => {
                self.advance();

                Ok(())
            }
            lookahead => Err(SyntaxError::UnexpectedToken(lookahead.unwrap())),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, SyntaxError> {
        if self.match_token(Token::Plus)? {
            self.consume(Token::Plus)?;
            let left = Box::new(self.parse_expr()?);
            let right = Box::new(self.parse_expr()?);

            return Ok(Expr::Plus(left, right));
        }

        Ok(Expr::Num(self.parse_int()?))
    }

    fn parse_int(&mut self) -> Result<i32, SyntaxError> {
        match self.lookahead.take() {
            None => Err(SyntaxError::UnexpectedEof),
            Some(Token::Int(num)) => {
                self.advance();

                Ok(num)
            }
            lookahead => Err(SyntaxError::UnexpectedToken(lookahead.unwrap())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_num() {
        let lexer = Lexer::new("1");
        let mut parser = Parser::new(lexer);

        assert_eq!(Expr::Num(1), parser.parse_expr().unwrap());
    }

    #[test]
    fn single_additional_expr() {
        let lexer = Lexer::new("+ 1 2");
        let mut parser = Parser::new(lexer);

        assert_eq!(
            Expr::Plus(Box::new(Expr::Num(1)), Box::new(Expr::Num(2))),
            parser.parse_expr().unwrap()
        );
    }
}
