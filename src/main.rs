use calcr::*;

fn main() {
    let stdin = std::io::stdin();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }

        let lexer = Lexer::new(&line);
        let mut parser = Parser::new(lexer);
        let res = parser.expr();
        println!("{res}");
    }
}
