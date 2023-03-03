pub fn lex(src: &str) -> Vec<Token> {
    let delimiters = "()+-*/^";
    let mut out = Vec::new();

    let mut iter = 0;
    while iter < src.len() {
        let Some(mut begin) = src[iter..].find(|c: char| !c.is_whitespace()) else {
            break;
        };

        begin += iter;

        let Some(mut end) = src[begin..]
                .find(|c: char| delimiters.contains(c) || c.is_whitespace()) else {
            push_token(&mut out, &src[begin..]);
            break;
        };

        if end == 0 {
            end += 1;
        }
        end += begin;
        push_token(&mut out, &src[begin..end]);

        iter = end;
    }

    out
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Lparen,
    Rparen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Number(f64),
}

fn push_token(tokens: &mut Vec<Token>, tkn_src: &str) {
    use Token::*;

    if let Ok(number) = tkn_src.parse::<f64>() {
        tokens.push(Number(number));
        return;
    }

    match tkn_src {
        "(" => tokens.push(Lparen),
        ")" => tokens.push(Rparen),
        "+" => tokens.push(Plus),
        "-" => tokens.push(Minus),
        "*" => tokens.push(Multiply),
        "/" => tokens.push(Divide),
        "^" => tokens.push(Power),
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_works() {
        let s = "((12 + 47) / (16 * 1 - 4^3))";

        use Token::*;
        assert_eq!(
            lex(&s),
            vec![
                Lparen,
                Lparen,
                Number(12.0),
                Plus,
                Number(47.0),
                Rparen,
                Divide,
                Lparen,
                Number(16.0),
                Multiply,
                Number(1.0),
                Minus,
                Number(4.0),
                Power,
                Number(3.0),
                Rparen,
                Rparen,
            ]
        );
    }
}
