use super::lexer::*;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn eval(&mut self) -> Result<i64, String> {
        self.lexer.next()?;

        self.expr()
    }

    fn expr(&mut self) -> Result<i64, String> {
        let lhs = self.mul_div()?;

        self.expr_tail(lhs)
    }

    fn expr_tail(&mut self, lhs: i64) -> Result<i64, String> {
        match self.lexer.token {
            Token::Plus => {
                self.lexer.consume(Token::Plus)?;
                let lhs = lhs + self.mul_div()?;

                self.expr_tail(lhs)
            }
            Token::Minus => {
                self.lexer.consume(Token::Minus)?;
                let lhs = lhs - self.mul_div()?;

                self.expr_tail(lhs)
            }
            _ => Ok(lhs),
        }
    }

    fn mul_div(&mut self) -> Result<i64, String> {
        let lhs = self.pow_fac()?;

        self.mul_div_tail(lhs)
    }

    fn mul_div_tail(&mut self, lhs: i64) -> Result<i64, String> {
        match self.lexer.token {
            Token::Mult => {
                self.lexer.consume(Token::Mult)?;
                let lhs = lhs * self.pow_fac()?;

                self.mul_div_tail(lhs)
            }
            Token::Div => {
                self.lexer.consume(Token::Div)?;
                let lhs = lhs / self.pow_fac()?;

                self.mul_div_tail(lhs)
            }
            _ => Ok(lhs),
        }
    }

    fn pow_fac(&mut self) -> Result<i64, String> {
        let lhs = self.new_expr()?;

        self.pow_fac_tail(lhs)
    }

    fn pow_fac_tail(&mut self, lhs: i64) -> Result<i64, String> {
        match self.lexer.token {
            Token::Pow => {
                self.lexer.consume(Token::Pow)?;
                let lhs = lhs.pow(self.new_expr()? as _);

                self.pow_fac_tail(lhs)
            }
            Token::Fact => {
                self.lexer.consume(Token::Fact)?;

                Ok((1..=lhs).product())
            }
            _ => Ok(lhs),
        }
    }

    fn new_expr(&mut self) -> Result<i64, String> {
        let value;
        match self.lexer.token {
            Token::Lparen => {
                self.lexer.consume(Token::Lparen)?;
                value = self.expr()?;
                self.lexer.consume(Token::Rparen)?;
            }
            Token::Number(n) => {
                value = n;
                self.lexer.consume(Token::Number(n))?;
            }
            ref u => return Err(format!("Unexpected token {u:?}")),
        };

        Ok(value)
    }
}
