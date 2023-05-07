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
        match parser.eval() {
            Ok(res) => println!("{res}"),
            Err(e) => println!("ERROR: {e}"),
        }
    }
}
