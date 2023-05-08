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

        self.expr(0)
    }

    fn expr(&mut self, prec_lvl: usize) -> Result<i64, String> {
        let mut lvalue: i64;

        use Token::*;
        match self.lexer.token {
            Number(n) => {
                self.lexer.consume(Number(n))?;
                lvalue = n;
            }
            Lparen => {
                self.lexer.consume(Lparen)?;
                lvalue = self.expr(Lparen.prec_lvl())?;
                self.lexer.consume(Rparen)?;
            }
            u => return Err(format!("Unexpected token {u:?}")),
        }

        while self.lexer.token.prec_lvl() >= prec_lvl {
            match self.lexer.token {
                Plus => {
                    self.lexer.consume(Plus)?;
                    lvalue += self.expr(Plus.prec_lvl() + 1)?;
                }
                Minus => {
                    self.lexer.consume(Minus)?;
                    lvalue -= self.expr(Minus.prec_lvl() + 1)?;
                }
                Mult => {
                    self.lexer.consume(Mult)?;
                    lvalue *= self.expr(Mult.prec_lvl() + 1)?;
                }
                Div => {
                    self.lexer.consume(Div)?;
                    lvalue /= self.expr(Div.prec_lvl() + 1)?;
                }
                Pow => {
                    self.lexer.consume(Pow)?;
                    lvalue = lvalue.pow(self.expr(Pow.prec_lvl() + 1)? as _);
                }
                Fact => {
                    self.lexer.consume(Fact)?;
                    lvalue = (1..=lvalue).product();
                }
                Rparen => {
                    if prec_lvl != Lparen.prec_lvl() {
                        return Err(format!("Unexpected token Rparen"));
                    } else {
                        return Ok(lvalue);
                    };
                }
                End => return Ok(lvalue),
                u => return Err(format!("Unexpected token {u:?}")),
            }
        }

        Ok(lvalue)
    }
}
