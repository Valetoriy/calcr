use super::lexer::*;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        lexer.next();
        Self { lexer }
    }

    pub fn expr(&mut self) -> i64 {
        let lhs = self.mul_div();

        self.expr_tail(lhs)
    }

    fn expr_tail(&mut self, lhs: i64) -> i64 {
        match self.lexer.token {
            Token::Plus => {
                self.lexer.consume(Token::Plus).unwrap();
                let lhs = lhs + self.mul_div();

                self.expr_tail(lhs)
            }
            Token::Minus => {
                self.lexer.consume(Token::Minus).unwrap();
                let lhs = lhs - self.mul_div();

                self.expr_tail(lhs)
            }
            _ => lhs,
        }
    }

    fn mul_div(&mut self) -> i64 {
        let lhs = self.pow_fac();

        return self.mul_div_tail(lhs);
    }

    fn mul_div_tail(&mut self, lhs: i64) -> i64 {
        match self.lexer.token {
            Token::Mult => {
                self.lexer.consume(Token::Mult).unwrap();
                let lhs = lhs * self.pow_fac();

                self.mul_div_tail(lhs)
            }
            Token::Div => {
                self.lexer.consume(Token::Div).unwrap();
                let lhs = lhs / self.pow_fac();

                self.mul_div_tail(lhs)
            }
            _ => lhs,
        }
    }

    fn pow_fac(&mut self) -> i64 {
        let lhs = self.new_expr();

        return self.pow_fac_tail(lhs);
    }

    fn pow_fac_tail(&mut self, lhs: i64) -> i64 {
        match self.lexer.token {
            Token::Pow => {
                self.lexer.consume(Token::Pow).unwrap();
                let lhs = lhs.pow(self.new_expr() as _);

                self.pow_fac_tail(lhs)
            }
            Token::Fact => {
                self.lexer.consume(Token::Fact).unwrap();
            
                (1..=lhs).product()
            }
            _ => lhs,
        }
    }

    fn new_expr(&mut self) -> i64 {
        let value;
        match self.lexer.token {
            Token::Lparen => {
                self.lexer.consume(Token::Lparen).unwrap();
                value = self.expr();
                self.lexer.consume(Token::Rparen).unwrap();
            }
            Token::Number(n) => {
                value = n;
                self.lexer.consume(Token::Number(n)).unwrap();
            }
            _ => panic!("Error: unknown symbol"),
        };

        value
    }
}
