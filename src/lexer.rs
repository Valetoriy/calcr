#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(i64),
    Lparen,
    Rparen,
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    Fact,

    End,
}

impl Token {
    pub fn prec_lvl(&self) -> usize {
        use Token::*;
        match self {
            Number(_) | Lparen | Rparen | End => 1, // не учавствуют в битве приоритетов
            Plus | Minus => 2,
            Mult | Div => 3,
            Pow | Fact => 4,
        }
    }
}

pub struct Lexer {
    pos: usize,
    chars: Vec<char>,
    pub token: Token,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Lexer {
            pos: 0,
            chars: src.chars().collect(),
            token: Token::Number(12), // инициализируется после первого `next()`
        }
    }

    pub fn next(&mut self) -> Result<(), String> {
        while let Some(c) = self.chars.get(self.pos) {
            if !c.is_whitespace() {
                break;
            }
            self.pos += 1;
        }

        let Some(&c) = self.chars.get(self.pos) else {
            self.token = End;
            return Ok(());
        };
        self.pos += 1;

        use Token::*;
        self.token = match c {
            n if c.is_numeric() => {
                let mut val = n.to_digit(10).unwrap();
                while let Some(n) = self.chars.get(self.pos) {
                    if !n.is_numeric() {
                        break;
                    }
                    val = val * 10 + n.to_digit(10).unwrap();
                    self.pos += 1;
                }

                Number(val as _)
            }
            '(' => Lparen,
            ')' => Rparen,
            '+' => Plus,
            '-' => Minus,
            '*' => Mult,
            '/' => Div,
            '^' => Pow,
            '!' => Fact,
            s => return Err(format!("Unexpected symbol {s}")),
        };

        Ok(())
    }

    pub fn consume(&mut self, token: Token) -> Result<(), String> {
        if self.token != token {
            return Err(format!("Expected {token:?}, got {:?}", self.token));
        }

        self.next()?;
        Ok(())
    }
}
